# Chapter15 Smart pointers

## Deref Coercion 與 Mutability 的轉換準則

By this rule, the compile will automatically find the suitable type which matchs the signature of the method or function. That makes coding much more convinent and easy.

- &T -> &U when T: Deref<Target=U>
- &mut T -> &mut U when T: DerefMut<Target=U>
- &mut T -> &U when T: Deref<Target=U>

This is because transformation must follow the borrow rule.

## Drop

Implement the trait `Drop` for the struct, and the struct would be automatically deallocate after it crosses the scope.

You can use `std::mem::drop` if you want to enforce the object to be dropped, for example, as you are dealing with the lock issues, you want to free the lock after completing the some tasks.
