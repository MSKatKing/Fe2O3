mod tag;
mod nbt;

pub use nbt::*;
pub use tag::NBTTag;

#[macro_export]
macro_rules! compound_nbt {
    ($($name:expr => $tag:expr),* $(,)?) => {
        NBTTag::Compound(
            vec![
                $(
                    ($name.into(), $tag.into()),
                )*
            ]
        )
    };
}

#[macro_export]
macro_rules! string_nbt {
    ($str:expr) => {
        NBTTag::String($str.into())
    };
}

#[macro_export]
macro_rules! list_nbt {
    (string, $($str:expr),*$(,)?) => {
        NBTTag::List(vec![
            $(
                NBTTag::String($str.into()),
            )*
        ])
    };
}

#[macro_export]
macro_rules! network_nbt {
    ($($name:expr => $tag:expr),* $(,)?) => {
        {
            let mut nbt = NBT::new(true);
            nbt.write_tag("", compound_nbt!(
                $(
                    $name => $tag,
                )*
            ));
            nbt
        }
    };
}
