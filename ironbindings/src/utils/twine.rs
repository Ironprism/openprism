use quote::ToTokens;

pub trait TwinePush<T> {
    fn push(&mut self, item: T);
}

pub struct Twine<T> {
    items: Vec<Vec<T>>,
}

impl<T> Twine<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn to_vec(self) -> Vec<T> {
        self.items.into_iter().flatten().collect()
    }
}

impl<T: ToTokens> ToTokens for Twine<T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.items.iter().for_each(|sub_vec| {
            sub_vec.iter().for_each(|item| {
                item.to_tokens(tokens);
            });
        });
    }
}

impl<T> TwinePush<T> for Twine<T> {
    fn push(&mut self, item: T) {
        self.items.push(vec![item]);
    }
}

impl<T> TwinePush<Vec<T>> for Twine<T> {
    fn push(&mut self, item: Vec<T>) {
        self.items.push(item);
    }
}

impl<T> TwinePush<Twine<T>> for Twine<T> {
    fn push(&mut self, item: Twine<T>) {
        self.items.extend(item.items);
    }
}

impl<T> Extend<T> for Twine<T> 
where
    Twine<T>: TwinePush<T>
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}
