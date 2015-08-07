use std::marker::PhantomData;

pub struct Tagged<S, B>(PhantomData<S>, B);

impl<S, B> AsRef<B> for Tagged<S, B> {
    fn as_ref(&self) -> &B { &self.1 }
}

impl<S, B> AsMut<B> for Tagged<S, B> {
    fn as_mut(&mut self) -> &mut B { &mut self.1 }
}

impl<S, B> Tagged<S, B> {

    pub fn new(b: B) -> Self {
        Tagged(PhantomData, b)
    }

    pub fn untag(self) -> B { self.1 }

    pub fn retag<T>(self) -> Tagged<T, B> {
        Tagged::new(self.1)
    }

}

#[test]
fn it_works() {
}
