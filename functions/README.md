# Functions

No argument, no return type:
fn fun_name() {
    //statements
}

multiple arguments , no return type
fn fun_name(argguments list) {
    // statments
}

multiple arguments , one return type
fn fun_name(argguments list)->return_type {
    // statments
}

multiple arguments , multiple return type
fn fun_name(argguments list)->(return_type1, return_Type2) {
    // statments
}

function inside a function
fn fun_name() {
    fn fun2() {
        //statements
    }
}