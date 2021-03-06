pub mod rect;
pub mod circle;
pub mod path;
pub mod group;
pub mod text;
pub mod word;
pub mod paint;
pub mod stroke;
pub mod fill;
pub mod translate;

pub use self::rect::*;
pub use self::circle::*;
pub use self::path::*;
pub use self::group::*;
pub use self::text::*;
pub use self::word::*;
pub use self::paint::*;
pub use self::stroke::*;
pub use self::fill::*;
pub use self::translate::*;

pub enum Shape {
    Rect(Rect),
    Circle(Circle),
    Path(Path),
    Group(Group),
    Text(Text),
    Word(Word),
}

pub trait Shapeable {
    fn rect(&self) -> Option<&Rect>;
    fn rect_mut(&mut self) -> Option<&mut Rect>;

    fn circle(&self) -> Option<&Circle>;
    fn circle_mut(&mut self) -> Option<&mut Circle>;

    fn path(&self) -> Option<&Path>;
    fn path_mut(&mut self) -> Option<&mut Path>;

    fn group(&self) -> Option<&Group>;
    fn group_mut(&mut self) -> Option<&mut Group>;

    fn text(&self) -> Option<&Text>;
    fn text_mut(&mut self) -> Option<&mut Text>;

    fn word(&self) -> Option<&Word>;
    fn word_mut(&mut self) -> Option<&mut Word>;
}

pub struct ShapeRef<'a>(pub &'a Shape);
pub struct ShapeRefMut<'a>(pub &'a mut Shape);

impl Shape {
    pub fn id(&self) -> Option<&str> {
        match self {
            Shape::Rect(r) => r.id(),
            Shape::Circle(c) => c.id(),
            Shape::Path(p) => p.id(),
            Shape::Group(g) => g.id(),
            Shape::Text(t) => t.id(),
            Shape::Word(w) => w.id(),
        }
    }

    #[inline]
    pub fn as_ref(&self) -> ShapeRef {
        ShapeRef(self)
    }

    #[inline]
    pub fn as_ref_mut(&mut self) -> ShapeRefMut {
        ShapeRefMut(self)
    }
}
impl Shapeable for Shape {
    #[inline]
    fn rect(&self) -> Option<&Rect> {
        match self {
            Shape::Rect(ref rect) => Some(rect),
            _ => None,
        }
    }

    #[inline]
    fn rect_mut(&mut self) -> Option<&mut Rect> {
        match self {
            Shape::Rect(ref mut rect) => Some(rect),
            _ => None,
        }
    }

    #[inline]
    fn circle(&self) -> Option<&Circle> {
        match self {
            Shape::Circle(ref circle) => Some(circle),
            _ => None,
        }
    }

    #[inline]
    fn circle_mut(&mut self) -> Option<&mut Circle> {
        match self {
            Shape::Circle(ref mut circle) => Some(circle),
            _ => None,
        }
    }

    #[inline]
    fn path(&self) -> Option<&Path> {
        match self {
            Shape::Path(ref path) => Some(path),
            _ => None,
        }
    }

    #[inline]
    fn path_mut(&mut self) -> Option<&mut Path> {
        match self {
            Shape::Path(ref mut path) => Some(path),
            _ => None,
        }
    }

    #[inline]
    fn group(&self) -> Option<&Group> {
        match self {
            Shape::Group(ref group) => Some(group),
            _ => None,
        }
    }

    #[inline]
    fn group_mut(&mut self) -> Option<&mut Group> {
        match self {
            Shape::Group(ref mut group) => Some(group),
            _ => None,
        }
    }

    #[inline]
    fn text(&self) -> Option<&Text> {
        match self {
            Shape::Text(ref text) => Some(text),
            _ => None,
        }
    }

    #[inline]
    fn text_mut(&mut self) -> Option<&mut Text> {
        match self {
            Shape::Text(ref mut text) => Some(text),
            _ => None,
        }
    }

    #[inline]
    fn word(&self) -> Option<&Word> {
        match self {
            Shape::Word(ref word) => Some(word),
            _ => None,
        }
    }

    #[inline]
    fn word_mut(&mut self) -> Option<&mut Word> {
        match self {
            Shape::Word(ref mut word) => Some(word),
            _ => None,
        }
    }
}

impl<'a> ShapeRef<'a> {
    #[inline]
    pub fn rect(&self) -> Option<&Rect> {
        self.0.rect()
    }

    #[inline]
    pub fn circle(&self) -> Option<&Circle> {
        self.0.circle()
    }

    #[inline]
    pub fn path(&self) -> Option<&Path> {
        self.0.path()
    }

    #[inline]
    pub fn group(&self) -> Option<&Group> {
        self.0.group()
    }

    #[inline]
    pub fn text(&self) -> Option<&Text> {
        self.0.text()
    }

    #[inline]
    pub fn word(&self) -> Option<&Word> {
        self.0.word()
    }
}

impl<'a> ShapeRefMut<'a> {
    #[inline]
    pub fn rect(&mut self) -> Option<&mut Rect> {
        self.0.rect_mut()
    }

    #[inline]
    pub fn circle(&mut self) -> Option<&mut Circle> {
        self.0.circle_mut()
    }

    #[inline]
    pub fn path(&mut self) -> Option<&mut Path> {
        self.0.path_mut()
    }

    #[inline]
    pub fn group(&mut self) -> Option<&mut Group> {
        self.0.group_mut()
    }

    #[inline]
    pub fn text(&mut self) -> Option<&mut Text> {
        self.0.text_mut()
    }

    #[inline]
    pub fn word(&mut self) -> Option<&mut Word> {
        self.0.word_mut()
    }
}

impl From<Rect> for Shape {
    fn from(rect: Rect) -> Self {
        Shape::Rect(rect)
    }
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Shape::Circle(circle)
    }
}

impl From<Path> for Shape {
    fn from(path: Path) -> Self {
        Shape::Path(path)
    }
}

impl From<Group> for Shape {
    fn from(group: Group) -> Self {
        Shape::Group(group)
    }
}

impl From<Text> for Shape {
    fn from(text: Text) -> Self {
        Shape::Text(text)
    }
}

impl From<Word> for Shape {
    fn from(word: Word) -> Self {
        Shape::Word(word)
    }
}

impl From<String> for Shape {
    fn from(text: String) -> Self {
        Shape::Word(Word { content: text, ..Default::default() })
    }
}