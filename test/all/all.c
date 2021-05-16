
int main() {
    int a = 0, b = 1;

    a = b * 3 + 1;

    if (a == 0)
        if (b == 0)
            return 3;
        else
            return 1;

}


//- gnc
//  - function
//    - data_type: "int"
//    - identifier: "main"
//    - function_parameter_list: "()"
//    - statement > declaration_statement
//      - data_type: "int"
//      - declaration
//        - identifier: "a"
//        - expression > logical_or_expression > logical_and_expression > inclusive_or_expression > exclusive_or_expression > bitwise_and_expression > equality_expression > comparison_expression > shift_expression > additive_expression > multiplicative_expression > unary_expression > int_literal > dec_literal: "0"
//      - declaration
//        - identifier: "b"
//        - expression > logical_or_expression > logical_and_expression > inclusive_or_expression > exclusive_or_expression > bitwise_and_expression > equality_expression > comparison_expression > shift_expression > additive_expression > multiplicative_expression > unary_expression > int_literal > dec_literal: "1"
//    - statement > expression > assignment_expression
//      - identifier: "a"
//      - assign_simple: "="
//      - expression > logical_or_expression > logical_and_expression > inclusive_or_expression > exclusive_or_expression > bitwise_and_expression > equality_expression > comparison_expression > shift_expression > additive_expression > multiplicative_expression
//        - unary_expression > identifier: "b"
//        - op_mul: "*"
//        - unary_expression > int_literal > dec_literal: "3"
//        - op_add: "+"
//        - unary_expression > int_literal > dec_literal: "1"
//    - statement > if_statement
//      - expression > logical_or_expression > logical_and_expression > inclusive_or_expression > exclusive_or_expression > bitwise_and_expression > equality_expression
//        - comparison_expression > shift_expression > additive_expression > multiplicative_expression > unary_expression > identifier: "a"
//        - op_eq: "=="
//        - comparison_expression > shift_expression > additive_expression > multiplicative_expression > unary_expression > int_literal > dec_literal: "0"
//      - statement > if_statement
//        - expression > logical_or_expression > logical_and_expression > inclusive_or_expression > exclusive_or_expression > bitwise_and_expression > equality_expression
//          - comparison_expression > shift_expression > additive_expression > multiplicative_expression > unary_expression > identifier: "b"
//          - op_eq: "=="
//          - comparison_expression > shift_expression > additive_expression > multiplicative_expression > unary_expression > int_literal > dec_literal: "0"
//        - statement > return_statement > expression > logical_or_expression > logical_and_expression > inclusive_or_expression > exclusive_or_expression > bitwise_and_expression > equality_expression > comparison_expression > shift_expression > additive_expression > multiplicative_expression > unary_expression > int_literal > dec_literal: "3"
//        - statement > return_statement > expression > logical_or_expression > logical_and_expression > inclusive_or_expression > exclusive_or_expression > bitwise_and_expression > equality_expression > comparison_expression > shift_expression > additive_expression > multiplicative_expression > unary_expression > int_literal > dec_literal: "1"
//  - EOI: ""