use crate::StackError as SmsfStackError;

pub trait InPlaceFnApplication {
    type Elem;

    fn unary_fn_in_place<U: FnOnce(&mut Self::Elem)>(
        &mut self,
        unary_fn: U,
    ) -> Result<(), SmsfStackError>;

    fn binary_fn_in_place_first_arg<U: FnOnce(&mut Self::Elem, &Self::Elem)>(
        &mut self,
        binary_fn: U,
    ) -> Result<(), SmsfStackError>;

    fn binary_fn_in_place_second_arg<U: FnOnce(&Self::Elem, &mut Self::Elem)>(
        &mut self,
        binary_fn: U,
    ) -> Result<(), SmsfStackError>;
}
