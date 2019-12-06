use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Name(Vec<String>);

impl From<String> for Name {
    fn from(v: String) -> Self {
        Name(vec![v])
    }
}

impl From<Vec<String>> for Name {
    fn from(v: Vec<String>) -> Self {
        Name(v)
    }
}

impl From<&'_ str> for Name {
    fn from(v: &str) -> Self {
        Name(vec![v.to_string()])
    }
}

impl From<Vec<&'_ str>> for Name {
    fn from(v: Vec<&'_ str>) -> Self {
        let v = v.into_iter().map(str::to_string).collect();
        Name(v)
    }
}

impl FromIterator<String> for Name {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Name(Vec::from_iter(iter))
    }
}

impl<'a> FromIterator<&'a str> for Name {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let v = iter.into_iter().map(str::to_string).collect::<Vec<_>>();
        Name(v)
    }
}

impl Extend<String> for Name {
    fn extend<T: IntoIterator<Item = String>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<'a> Extend<&'a str> for Name {
    fn extend<T: IntoIterator<Item = &'a str>>(&mut self, iter: T) {
        self.0.extend(iter.into_iter().map(str::to_string))
    }
}
