use std::mem;
use std::any::Any;
use std::rc::Rc;
use egml::{
    ModelComponent, ShouldChangeView, Viewable, Drawable, DrawableChilds,
    Node, NodeDefaults, Shape, ChildrenProcessed,
};
use controller::InputEvent;

trait AsAny {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn downcast_ref<U: 'static>(&self) -> Option<&U>;

    fn downcast_mut<U: 'static>(&mut self) -> Option<&mut U>;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn downcast_ref<U: 'static>(&self) -> Option<&U> {
        self.as_any().downcast_ref::<U>()
    }

    fn downcast_mut<U: 'static>(&mut self) -> Option<&mut U> {
        self.as_any_mut().downcast_mut::<U>()
    }
}

#[derive(Default)]
pub struct Comp {
    pub model: Option<Box<dyn Any>>,
    pub props: Option<Box<dyn Any>>,
    pub view_node: Option<Box<dyn Any>>,
    pub defaults: Option<Rc<NodeDefaults>>,
    pub resolver: Option<fn(&mut Comp) -> ChildrenProcessed>,
    pub drawer: Option<fn(&Comp) -> &dyn Drawable>,
    pub inputer: Option<fn(&mut Comp, InputEvent) -> ShouldChangeView>,
}

impl Comp {
    /// This method prepares a generator to make a new instance of the `Component`.
    pub fn lazy<MYMC>() -> (<MYMC as ModelComponent>::Properties, Self)
        where
            MYMC: ModelComponent + Viewable<MYMC>,
    {
        (Default::default(), Default::default())
    }

    pub fn new<MYMC>(props: <MYMC as ModelComponent>::Properties) -> Self
        where
            MYMC: ModelComponent + Viewable<MYMC>,
    {
        let mut comp = Comp::default();
        comp.init::<MYMC>(props);
        comp
    }

    /// Create model and attach properties associated with the component.
    pub fn init<MYMC>(&mut self, props: <MYMC as ModelComponent>::Properties)
        where
            MYMC: ModelComponent + Viewable<MYMC>,
    {
        let model = <MYMC as ModelComponent>::create(&props);
        let node = model.view();
        self.model = Some(Box::new(model));
        self.view_node = Some(Box::new(node));
        self.props = Some(Box::new(props));
        self.resolver = Some(|comp: &mut Comp| {
            let defaults = comp.clone_defaults();
            comp.view_node_mut::<MYMC>().resolve(defaults)
        });
        self.drawer = Some(|comp: &Comp| {
            comp.view_node::<MYMC>() as &dyn Drawable
        });
        self.inputer = Some(|comp: &mut Comp, event: InputEvent| {
            let mut view_node = mem::replace(&mut comp.view_node, None)
                .expect("Inputer can't extract node");
            {
                let defaults = comp.clone_defaults();
                let model = comp.model_mut::<MYMC>();
                let result = (*view_node)
                    .downcast_mut::<Node<MYMC>>().expect("Inputer can't downcast node")
                    .input(event, model);
                if result {
                    let mut new_node = model.view();
                    new_node.resolve(defaults);
                    view_node = Box::new(new_node);
                }
            }
            mem::replace(&mut comp.view_node, Some(view_node));
            false
        });
    }

    pub fn resolve(&mut self, defaults: Option<Rc<NodeDefaults>>) -> ChildrenProcessed {
        self.defaults = defaults;
        (self.resolver.expect("Can't resolve with uninitialized resolver"))(
            self
        )
    }

    pub fn view_node<M: ModelComponent>(&self) -> &Node<M> {
        let node = self.view_node.as_ref().expect("Can't downcast node - it is None");
        (*(*node)).downcast_ref::<Node<M>>().expect("Can't downcast node")
    }

    pub fn view_node_mut<M: ModelComponent>(&mut self) -> &mut Node<M> {
        let node = self.view_node.as_mut().expect("Can't downcast node - it is None");
        (*(*node)).downcast_mut::<Node<M>>().expect("Can't downcast node")
    }

    pub fn model<M: ModelComponent>(&self) -> &M {
        let model = self.model.as_ref().expect("Can't downcast model - it is None");
        (*(*model)).downcast_ref::<M>().expect("Can't downcast model")
    }

    pub fn model_mut<M: ModelComponent>(&mut self) -> &mut M {
        let model = self.model.as_mut().expect("Can't downcast model - it is None");
        (*(*model)).downcast_mut::<M>().expect("Can't downcast model")
    }

    pub fn input(&mut self, event: InputEvent) -> ShouldChangeView {
        self.inputer.map(|inputer| {
            inputer(self, event)
        }).unwrap_or(false)
    }

    pub fn clone_defaults(&self) -> Option<Rc<NodeDefaults>> {
        self.defaults.as_ref().map(|d| Rc::clone(d))
    }
}

impl Drawable for Comp {
    fn shape(&self) -> Option<&Shape> {
        self.drawer.and_then(|drawer| {
            drawer(self).shape()
        })
    }

    fn childs(&self) -> Option<DrawableChilds> {
        self.drawer.and_then(|drawer| {
            drawer(self).childs()
        })
    }
}

/// Converts property and attach lazy components to it.
pub trait Transformer<MC: ModelComponent, FROM, TO> {
    /// Transforms one type to another.
    fn transform(&mut self, from: FROM) -> TO;
}

impl<MC, T> Transformer<MC, T, T> for Comp
    where
        MC: ModelComponent,
{
    fn transform(&mut self, from: T) -> T {
        from
    }
}

impl<'a, MC, T> Transformer<MC, &'a T, T> for Comp
    where
        MC: ModelComponent,
        T: Clone,
{
    fn transform(&mut self, from: &'a T) -> T {
        from.clone()
    }
}

impl<'a, MC> Transformer<MC, &'a str, String> for Comp
    where
        MC: ModelComponent,
{
    fn transform(&mut self, from: &'a str) -> String {
        from.to_owned()
    }
}
