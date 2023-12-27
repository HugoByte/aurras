use super::*;

#[macro_export]
macro_rules! make_input_struct {
    (
        $x:ident,
        [$(
            $(#[$default_derive:stmt])?
            $visibility:vis $element:ident : $ty:ty),*],
        [$($der:ident),*]
) => {
        #[derive($($der),*)]
            pub struct $x { 
            $(
                $(#[serde(default=$default_derive)])?
                $visibility  $element: $ty
            ),*
        }
    }
}

#[macro_export]
macro_rules! make_main_struct {
    (
        $name:ident,
        $input:ty,
        [$($der:ident),*],
        [$($key:ident : $val:expr),*],
        $output_field: ident
) => {
        #[derive($($der),*)]
        $(
            #[$key = $val]
        )*
        pub struct $name {
            action_name: String,
            pub input: $input,
            pub output: Value,
            pub mapout: Value
        }
        impl $name{
            pub fn output(&self) -> Value {
                self.$output_field.clone()
            }
        }
    }
}

#[macro_export]
macro_rules! impl_new {
    (
        $name:ident,
        $input:ident,
        []
    ) => {
        impl $name{
            pub fn new(action_name:String) -> Self{
                Self{
                    action_name,
                    input: $input{
                        ..Default::default()
                    },
                    ..Default::default()
                }      
            }
        }
    };
    (
        $name:ident,
        $input:ident,
        [$($element:ident : $ty:ty),*]
    ) => {
        impl $name{
            pub fn new($( $element: $ty),*, action_name:String) -> Self{
                Self{
                    action_name,
                    input: $input{
                        $($element),*,
                        ..Default::default()
                    },
                    ..Default::default()
                }      
            }
        }
    }
}

#[macro_export]
macro_rules! impl_setter {
    (
        $name:ty,
        [$($element:ident : $key:expr),*]
    ) => {
        impl $name{
            pub fn setter(&mut self, value: Value) {
                $(
                    let val = value.get($key).unwrap();
                    self.input.$element = serde_json::from_value(val.clone()).unwrap();
                )*
            }
        }
    }
}

#[macro_export]
macro_rules! impl_map_setter {
    (
        $name:ty,
        $element:ident : $key:expr,  
        $typ_name : ty,
        $out:expr
    ) => {
        impl $name {
            pub fn setter(&mut self, val: Value) {
                
                    let value = val.get($key).unwrap();
                    let value = serde_json::from_value::<Vec<$typ_name>>(value.clone()).unwrap();
                    let mut map: HashMap<_, _> = value
                        .iter()
                        .map(|x| {
                            self.input.$element = x.to_owned() as $typ_name;
                            self.run();
                            (x.to_owned(), self.output.get($out).unwrap().to_owned())
                        })
                        .collect();
                    self.mapout = to_value(map).unwrap();
                
            }
        }
    }
    }

#[macro_export]
macro_rules! impl_concat_setter {
    (
        $name:ty,
        $input:ident
    ) => {
        impl $name{
            pub fn setter(&mut self, val: Value) {
                
                    let val: Vec<Value> = serde_json::from_value(val).unwrap();
                    let res = join_hashmap(
                        serde_json::from_value(val[0].to_owned()).unwrap(),
                        serde_json::from_value(val[1].to_owned()).unwrap(),
                    );
                    self.input.$input = res;
            }
        }
    }
}

#[allow(unused)]
#[macro_export]
macro_rules! impl_combine_setter {
    (
        $name:ty,
        [$(
            $(($value_input:ident))?
            $([$index:expr])?
            $element:ident : $key:expr),*]
    ) => {
        impl $name{
            pub fn setter(&mut self, value: Value) {

                let value: Vec<Value> = serde_json::from_value(value).unwrap();
                $(
                    if stringify!($($value_input)*).is_empty(){
                        let val = value[$($index)*].get($key).unwrap();
                        self.input.$element = serde_json::from_value(val.clone()).unwrap();
                    }else{
                        self.input.$element = serde_json::from_value(value[$($index)*].to_owned()).unwrap();
                    }
                )*
            }
        }
    }
}

