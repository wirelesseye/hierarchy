#[macro_export]
macro_rules! class_utils {
    (@impl_base $vis:vis $struct_name:ident {
        $(let $field_name:ident : $field_type:ty ;)*

        $(pub final fn $pub_final_fn_name:ident $pub_final_params:tt -> $pub_final_return_type:ty {
            $($pub_final_fn_stmt:stmt);* $(;)?
        })*

        $(pub fn $pub_fn_name:ident $pub_params:tt -> $pub_return_type:ty {
            $($pub_fn_stmt:stmt);* $(;)?
        })*

        $(fn $pri_fn_name:ident $pri_params:tt -> $pri_return_type:ty {
            $($pri_fn_stmt:stmt);* $(;)?
        })*
    }) => {
        hierarchy::paste! {
            impl $struct_name {
                $(fn [<get_ $field_name>](&self) -> &$field_type {
                    &self.$field_name
                })*

                $(fn $pub_final_fn_name $pub_final_params -> $pub_final_return_type {
                    $($pub_final_fn_stmt);*
                })*

                $(fn $pri_fn_name $pri_params -> $pri_return_type {
                    $($pri_fn_stmt);*
                })*
            }

            $vis trait [<$struct_name Trait>] {
                fn [<get_ $struct_name:snake>](&self) -> &$struct_name;

                $(fn [<get_ $field_name>](&self) -> &$field_type {
                    self.[<get_ $struct_name:snake>]().[<get_ $field_name>]()
                })*

                $(fn $pub_fn_name $pub_params -> $pub_return_type {
                    $($pub_fn_stmt);*
                })*
            }

            impl [<$struct_name Trait>] for $struct_name {
                fn [<get_ $struct_name:snake>](&self) -> &$struct_name {
                    self
                }
            }
        }
    };
}

#[macro_export]
macro_rules! class {
    (
        $vis:vis $struct_name:ident extends $super_name:ident $(< $inherit_chain:tt)* {
            $(let $field_name:ident : $field_type:ty ;)*

            $(override $override_name:ident {
                $(pub fn $override_fn_name:ident $override_params:tt -> $override_return_type:ty {
                    $($override_fn_stmt:stmt);* $(;)?
                })*
            })*

            $(pub final fn $pub_final_fn_name:ident $pub_final_params:tt -> $pub_final_return_type:ty {
                $($pub_final_fn_stmt:stmt);* $(;)?
            })*

            $(pub fn $pub_fn_name:ident $pub_params:tt -> $pub_return_type:ty {
                $($pub_fn_stmt:stmt);* $(;)?
            })*

            $(fn $pri_fn_name:ident $pri_params:tt -> $pri_return_type:ty {
                $($pri_fn_stmt:stmt);* $(;)?
            })*
        }
    ) => {
        hierarchy::paste! {
            $vis struct $struct_name {
                [<$super_name:snake>]: $super_name,
                $($field_name: $field_type),*
            }
        }

        hierarchy::class_utils!(@impl_base $vis $struct_name {
            $(let $field_name : $field_type ;)*

            $(pub final fn $pub_final_fn_name $pub_final_params -> $pub_final_return_type {
                $($pub_final_fn_stmt);*
            })*

            $(pub fn $pub_fn_name $pub_params -> $pub_return_type {
                $($pub_fn_stmt);*
            })*

            $(fn $pri_fn_name $pri_params -> $pri_return_type {
                $($pri_fn_stmt);*
            })*
        });

        hierarchy::inherit!(
            $struct_name,
            $super_name $(< $inherit_chain)*,
            $(override $override_name {
                $(fn $override_fn_name $override_params -> $override_return_type {
                    $($override_fn_stmt);*
                })*
            })*
        );
    };
    (
        $vis:vis $struct_name:ident {
            $(let $field_name:ident : $field_type:ty ;)*

            $(pub final fn $pub_final_fn_name:ident $pub_final_params:tt -> $pub_final_return_type:ty {
                $($pub_final_fn_stmt:stmt);* $(;)?
            })*

            $(pub fn $pub_fn_name:ident $pub_params:tt -> $pub_return_type:ty {
                $($pub_fn_stmt:stmt);* $(;)?
            })*

            $(fn $pri_fn_name:ident $pri_params:tt -> $pri_return_type:ty {
                $($pri_fn_stmt:stmt);* $(;)?
            })*
        }
    ) => {
        $vis struct $struct_name {
            $($field_name: $field_type),*
        }

        hierarchy::class_utils!(@impl_base $vis $struct_name {
            $(let $field_name : $field_type ;)*

            $(pub final fn $pub_final_fn_name $pub_final_params -> $pub_final_return_type {
                $($pub_final_fn_stmt);*
            })*

            $(pub fn $pub_fn_name $pub_params -> $pub_return_type {
                $($pub_fn_stmt);*
            })*

            $(fn $pri_fn_name $pri_params -> $pri_return_type {
                $($pri_fn_stmt);*
            })*
        });
    };
}

#[macro_export]
macro_rules! final_class {
    (
        $vis:vis $struct_name:ident extends $inherit_name:ident {
            $(let $field_name:ident : $field_type:ty;)*

            $($func_vis:vis fn $func_name:ident $func_params:tt -> $return_type:ty {
                $($fn_stmt:stmt);* $(;)?
            })*
        }
    ) => {
        hierarchy::paste! {
            $vis struct $struct_name {
                [<$inherit_name:snake>]: $inherit_name,
                $($field_name: $field_type),*
            }

            impl [<$inherit_name Trait>] for $struct_name {
                fn [<get_ $inherit_name:snake>](&self) -> &$inherit_name {
                    &self.[<$inherit_name:snake>]
                }
            }

            impl $struct_name {
                $($func_vis fn $func_name $func_params -> $return_type {
                    $($fn_stmt);*
                })*
            }
        }
    };
    (
        $vis:vis $struct_name:ident extends $inherit_name:ident {
            $(let $field_name:ident : $field_type:ty;)*

            override {
                $(fn $override_func_name:ident $override_func_params:tt -> $override_return_type:ty {
                    $($override_fn_stmt:stmt);* $(;)?
                })*
            }

            $($func_vis:vis fn $func_name:ident $func_params:tt -> $return_type:ty {
                $($fn_stmt:stmt);* $(;)?
            })*
        }
    ) => {
        hierarchy::paste! {
            $vis struct $struct_name {
                [<$inherit_name:snake>]: $inherit_name,
                $($field_name: $field_type),*
            }

            impl [<$inherit_name Trait>] for $struct_name {
                fn [<get_ $inherit_name:snake>](&self) -> &$inherit_name {
                    &self.[<$inherit_name:snake>]
                }

                $(fn $override_func_name $override_func_params -> $override_return_type {
                    $($override_fn_stmt);*
                })*
            }

            impl $struct_name {
                $($func_vis fn $func_name $func_params -> $return_type {
                    $($fn_stmt);*
                })*
            }
        }
    };
}
