use typename::TypeName;
use typename::TypeNameTrait;

#[derive(TypeName)]
struct Hello;

fn main() {
    let hello = Hello;
    dbg!(hello.type_name());
}
