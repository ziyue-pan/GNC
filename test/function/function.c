int main() {
    return foo(0, 1);
}

int foo(int a, int b) {
    return a + b;
}


int bar(int a, int b, int c) {
    int rst = 0;
    while (a < b) {
        rst <<= c;
        a += 1;
    }
    return rst;
}


//- gnc
//  - function
//    - data_type: "int"
//    - identifier: "main"
//    - function_parameter_list: "()"
//    - statement > return_statement > expression > logical_or_expression > logical_and_expression > inclusive_or_expression > exclusive_or_expression > bitwise_and_expression > equality_expression > comparison_expression > shift_expression > additive_expression > multiplicative_expression > unary_expression > function_call
//      - identifier: "foo"
//      - expression > logical_or_expression > logical_and_expression > inclusive_or_expression > exclusive_or_expression > bitwise_and_expression > equality_expression > comparison_expression > shift_expression > additive_expression > multiplicative_expression > unary_expression > int_literal > dec_literal: "0"
//      - expression > logical_or_expression > logical_and_expression > inclusive_or_expression > exclusive_or_expression > bitwise_and_expression > equality_expression > comparison_expression > shift_expression > additive_expression > multiplicative_expression > unary_expression > int_literal > dec_literal: "1"
//  - function
//    - data_type: "int"
//    - identifier: "foo"
//    - function_parameter_list
//      - function_parameter
//        - data_type: "int"
//        - identifier: "a"
//      - function_parameter
//        - data_type: "int"
//        - identifier: "b"
//    - statement > return_statement > expression > logical_or_expression > logical_and_expression > inclusive_or_expression > exclusive_or_expression > bitwise_and_expression > equality_expression > comparison_expression > shift_expression > additive_expression > multiplicative_expression
//      - unary_expression > identifier: "a"
//      - op_add: "+"
//      - unary_expression > identifier: "b"
//  - function
//    - data_type: "int"
//    - identifier: "bar"
//    - function_parameter_list
//      - function_parameter
//        - data_type: "int"
//        - identifier: "a"
//      - function_parameter
//        - data_type: "int"
//        - identifier: "b"
//      - function_parameter
//        - data_type: "int"
//        - identifier: "c"
//    - statement > declaration_statement
//      - data_type: "int"
//      - declaration
//        - identifier: "rst"
//        - expression > logical_or_expression > logical_and_expression > inclusive_or_expression > exclusive_or_expression > bitwise_and_expression > equality_expression > comparison_expression > shift_expression > additive_expression > multiplicative_expression > unary_expression > int_literal > dec_literal: "0"
//    - statement > while_statement
//      - expression > logical_or_expression > logical_and_expression > inclusive_or_expression > exclusive_or_expression > bitwise_and_expression > equality_expression > comparison_expression
//        - shift_expression > additive_expression > multiplicative_expression > unary_expression > identifier: "a"
//        - op_lt: "<"
//        - shift_expression > additive_expression > multiplicative_expression > unary_expression > identifier: "b"
//      - statement > block_statement
//        - statement > expression > assignment_expression
//          - identifier: "rst"
//          - assign_shift_left: "<<="
//          - expression > logical_or_expression > logical_and_expression > inclusive_or_expression > exclusive_or_expression > bitwise_and_expression > equality_expression > comparison_expression > shift_expression > additive_expression > multiplicative_expression > unary_expression > identifier: "c"
//        - statement > expression > assignment_expression
//          - identifier: "a"
//          - assign_add: "+="
//          - expression > logical_or_expression > logical_and_expression > inclusive_or_expression > exclusive_or_expression > bitwise_and_expression > equality_expression > comparison_expression > shift_expression > additive_expression > multiplicative_expression > unary_expression > int_literal > dec_literal: "1"
//    - statement > return_statement > expression > logical_or_expression > logical_and_expression > inclusive_or_expression > exclusive_or_expression > bitwise_and_expression > equality_expression > comparison_expression > shift_expression > additive_expression > multiplicative_expression > unary_expression > identifier: "rst"
//  - EOI: ""