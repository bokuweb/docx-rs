/// Macro to delegate methods to a specified field
/// 
/// This macro generates methods that delegate to methods on a field of the struct.
/// Each delegated method will call the corresponding method on the specified field
/// and return `self` for method chaining.
/// 
/// # Example
/// 
/// ```rust,ignore
/// use docx_rs::delegate_to_field;
/// 
/// struct Wrapper {
///     inner: SomeType,
/// }
/// 
/// impl Wrapper {
///     delegate_to_field! {
///         inner =>
///         method1(param: i32) -> Self,
///         method2(a: String, b: bool) -> Self,
///         method3() -> Self,
///     }
/// }
/// ```
#[macro_export]
macro_rules! delegate_to_field {
    ($field:ident => $($method:ident($($param:ident: $param_type:ty),*) -> Self),* $(,)?) => {
        $(
            pub fn $method(mut self, $($param: $param_type),*) -> Self {
                self.$field = self.$field.$method($($param),*);
                self
            }
        )*
    };
}

/// Macro to delegate getter methods to a specified field
/// 
/// This macro generates getter methods that delegate to methods on a field of the struct.
/// Each delegated method will call the corresponding method on the specified field
/// and return the result directly.
/// 
/// # Example
/// 
/// ```rust,ignore
/// use docx_rs::delegate_getters_to_field;
/// 
/// struct Wrapper {
///     inner: SomeType,
/// }
/// 
/// impl Wrapper {
///     delegate_getters_to_field! {
///         inner =>
///         get_value() -> i32,
///         get_name() -> String,
///         is_active() -> bool,
///     }
/// }
/// ```
#[macro_export]
macro_rules! delegate_getters_to_field {
    ($field:ident => $($method:ident() -> $return_type:ty),* $(,)?) => {
        $(
            pub fn $method(&self) -> $return_type {
                self.$field.$method()
            }
        )*
    };
}
