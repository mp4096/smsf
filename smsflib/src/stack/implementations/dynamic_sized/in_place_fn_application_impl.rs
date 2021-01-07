use super::DynamicSizedStack;
use crate::stack::InPlaceFnApplication;
use crate::Error as SmsfError;

impl<T: Clone> InPlaceFnApplication for DynamicSizedStack<T> {
    type Elem = T;

    /// Appy a unary operation to the lowest register in-place.
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::new();
    /// let res = stack.unary_fn_in_place(|x: &mut u32| {*x += 10; } );
    ///
    /// assert_eq!(res, Err(smsflib::Error::NotEnoughOperands { num_required: 1, num_available: 0 }));
    /// ```
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::clone_from_slice(&[3, 2, 1]);
    /// let res = stack.unary_fn_in_place(|x: &mut u32| {*x += 10; } );
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(stack.len(), 3);
    /// assert_eq!(stack.get(0), Some(&11));
    /// assert_eq!(stack.get(1), Some(&2));
    /// assert_eq!(stack.get(2), Some(&3));
    /// ```
    ///
    fn unary_fn_in_place<U: FnOnce(&mut Self::Elem)>(
        &mut self,
        unary_fn: U,
    ) -> Result<(), SmsfError> {
        match self.container.last_mut() {
            Some(first_elem_mut_ref) => {
                unary_fn(first_elem_mut_ref);
                Ok(())
            }
            None => Err(SmsfError::NotEnoughOperands {
                num_required: 1,
                num_available: 0,
            }),
        }
    }

    /// Appy a binary operation to the two lowermost registers, consuming them and pushing the result.
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::clone_from_slice(&[3, 2, 10]);
    /// let res = stack.binary_fn_in_place_first_arg(|x: &mut u32, y: &u32| {*x *= y; } );
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(stack.len(), 2);
    /// assert_eq!(stack.get(0), Some(&20));
    /// assert_eq!(stack.get(1), Some(&3));
    /// ```
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::clone_from_slice(&[10]);
    /// let res = stack.binary_fn_in_place_first_arg(|x: &mut u32, y: &u32| {*x *= y; } );
    ///
    /// assert_eq!(res, Err(smsflib::Error::NotEnoughOperands{ num_required: 2, num_available: 1 }));
    /// ```
    ///
    fn binary_fn_in_place_first_arg<U: FnOnce(&mut Self::Elem, &Self::Elem)>(
        &mut self,
        binary_fn: U,
    ) -> Result<(), SmsfError> {
        if self.len() >= 2 {
            // '.unwrap()' is safe here
            let idx_penultimate = self.len() - 2;
            let penultimate_item = self.container.remove(idx_penultimate);
            binary_fn(self.container.last_mut().unwrap(), &penultimate_item);
            Ok(())
        } else {
            Err(SmsfError::NotEnoughOperands {
                num_required: 2,
                num_available: self.len(),
            })
        }
    }

    /// Appy a binary operation to the two lowermost registers, consuming them and pushing the result.
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::clone_from_slice(&[3, 20, 10]);
    /// let res = stack.binary_fn_in_place_second_arg(|x: &u32, y: &mut u32| {*y /= x; } );
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(stack.len(), 2);
    /// assert_eq!(stack.get(0), Some(&2));
    /// assert_eq!(stack.get(1), Some(&3));
    /// ```
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::clone_from_slice(&[10]);
    /// let res = stack.binary_fn_in_place_second_arg(|x: &u32, y: &mut u32| {*y /= x; } );
    ///
    /// assert_eq!(res, Err(smsflib::Error::NotEnoughOperands{ num_required: 2, num_available: 1 }));
    /// ```
    ///
    fn binary_fn_in_place_second_arg<U: FnOnce(&Self::Elem, &mut Self::Elem)>(
        &mut self,
        binary_fn: U,
    ) -> Result<(), SmsfError> {
        if self.len() >= 2 {
            // '.unwrap()'s are safe here
            let ultimate_item = self.container.pop().unwrap();
            binary_fn(&ultimate_item, self.container.last_mut().unwrap());
            Ok(())
        } else {
            Err(SmsfError::NotEnoughOperands {
                num_required: 2,
                num_available: self.len(),
            })
        }
    }
}
