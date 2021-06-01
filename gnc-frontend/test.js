const data = {"error":false,"error_message":"","parse_tree":{"id":"gnc(0,83)","label":"gnc","children":[{"id":"function(0,39)","label":"function","children":[{"id":"data_type(0,3)","label":"data_type","children":[{"id":"int(0,3)","label":"int","children":[]}]},{"id":"identifier(4,8)","label":"identifier","children":[]},{"id":"function_parameter_list(8,10)","label":"function_parameter_list","children":[]},{"id":"statement(17,23)","label":"statement","children":[{"id":"expression(17,22)","label":"expression","children":[{"id":"assignment_expression(17,22)","label":"assignment_expression","children":[{"id":"identifier(17,18)","label":"identifier","children":[]},{"id":"assign_simple(19,20)","label":"assign_simple","children":[]},{"id":"expression(21,22)","label":"expression","children":[{"id":"logical_or_expression(21,22)","label":"logical_or_expression","children":[{"id":"logical_and_expression(21,22)","label":"logical_and_expression","children":[{"id":"inclusive_or_expression(21,22)","label":"inclusive_or_expression","children":[{"id":"exclusive_or_expression(21,22)","label":"exclusive_or_expression","children":[{"id":"bitwise_and_expression(21,22)","label":"bitwise_and_expression","children":[{"id":"equality_expression(21,22)","label":"equality_expression","children":[{"id":"comparison_expression(21,22)","label":"comparison_expression","children":[{"id":"shift_expression(21,22)","label":"shift_expression","children":[{"id":"additive_expression(21,22)","label":"additive_expression","children":[{"id":"multiplicative_expression(21,22)","label":"multiplicative_expression","children":[{"id":"unary_expression(21,22)","label":"unary_expression","children":[{"id":"int_literal(21,22)","label":"int_literal","children":[{"id":"dec_literal(21,22)","label":"dec_literal","children":[]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]},{"id":"statement(28,37)","label":"statement","children":[{"id":"return_statement(28,36)","label":"return_statement","children":[{"id":"expression(35,36)","label":"expression","children":[{"id":"logical_or_expression(35,36)","label":"logical_or_expression","children":[{"id":"logical_and_expression(35,36)","label":"logical_and_expression","children":[{"id":"inclusive_or_expression(35,36)","label":"inclusive_or_expression","children":[{"id":"exclusive_or_expression(35,36)","label":"exclusive_or_expression","children":[{"id":"bitwise_and_expression(35,36)","label":"bitwise_and_expression","children":[{"id":"equality_expression(35,36)","label":"equality_expression","children":[{"id":"comparison_expression(35,36)","label":"comparison_expression","children":[{"id":"shift_expression(35,36)","label":"shift_expression","children":[{"id":"additive_expression(35,36)","label":"additive_expression","children":[{"id":"multiplicative_expression(35,36)","label":"multiplicative_expression","children":[{"id":"unary_expression(35,36)","label":"unary_expression","children":[{"id":"identifier(35,36)","label":"identifier","children":[]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]},{"id":"global_variable(42,53)","label":"global_variable","children":[{"id":"data_type(42,45)","label":"data_type","children":[{"id":"int(42,45)","label":"int","children":[]}]},{"id":"identifier(46,47)","label":"identifier","children":[]},{"id":"expression(51,52)","label":"expression","children":[{"id":"logical_or_expression(51,52)","label":"logical_or_expression","children":[{"id":"logical_and_expression(51,52)","label":"logical_and_expression","children":[{"id":"inclusive_or_expression(51,52)","label":"inclusive_or_expression","children":[{"id":"exclusive_or_expression(51,52)","label":"exclusive_or_expression","children":[{"id":"bitwise_and_expression(51,52)","label":"bitwise_and_expression","children":[{"id":"equality_expression(51,52)","label":"equality_expression","children":[{"id":"comparison_expression(51,52)","label":"comparison_expression","children":[{"id":"shift_expression(51,52)","label":"shift_expression","children":[{"id":"additive_expression(51,52)","label":"additive_expression","children":[{"id":"multiplicative_expression(51,52)","label":"multiplicative_expression","children":[{"id":"unary_expression(51,52)","label":"unary_expression","children":[{"id":"int_literal(51,52)","label":"int_literal","children":[{"id":"dec_literal(51,52)","label":"dec_literal","children":[]}]}]}]}]}]}]}]}]}]}]}]}]}]}]},{"id":"function(55,82)","label":"function","children":[{"id":"data_type(55,58)","label":"data_type","children":[{"id":"int(55,58)","label":"int","children":[]}]},{"id":"identifier(59,62)","label":"identifier","children":[]},{"id":"function_parameter_list(62,64)","label":"function_parameter_list","children":[]},{"id":"statement(71,80)","label":"statement","children":[{"id":"return_statement(71,79)","label":"return_statement","children":[{"id":"expression(78,79)","label":"expression","children":[{"id":"logical_or_expression(78,79)","label":"logical_or_expression","children":[{"id":"logical_and_expression(78,79)","label":"logical_and_expression","children":[{"id":"inclusive_or_expression(78,79)","label":"inclusive_or_expression","children":[{"id":"exclusive_or_expression(78,79)","label":"exclusive_or_expression","children":[{"id":"bitwise_and_expression(78,79)","label":"bitwise_and_expression","children":[{"id":"equality_expression(78,79)","label":"equality_expression","children":[{"id":"comparison_expression(78,79)","label":"comparison_expression","children":[{"id":"shift_expression(78,79)","label":"shift_expression","children":[{"id":"additive_expression(78,79)","label":"additive_expression","children":[{"id":"multiplicative_expression(78,79)","label":"multiplicative_expression","children":[{"id":"unary_expression(78,79)","label":"unary_expression","children":[{"id":"int_literal(78,79)","label":"int_literal","children":[{"id":"dec_literal(78,79)","label":"dec_literal","children":[]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]},{"id":"EOI(83,83)","label":"EOI","children":[]}]},"ast":[{"Function":["Int","main",[],[{"Assignment":["Simple","a",{"IntLiteral":3}]},{"ReturnStatement":{"Identifier":"a"}}]]},{"GlobalDeclaration":["Int","a",{"IntLiteral":0}]},{"Function":["Int","foo",[],[{"ReturnStatement":{"IntLiteral":0}}]]}]}
const ast = data['ast']

const node2tree = (astNode) => {
    let nodeList = []
    for (const node of astNode) {
        for (const x in node) { // Actually there's only one entry inside
            id += 1
            let treeNode = {
                id: id.toString(),
                label: x,
                attrs: {},
                children: []
            }
            switch (x) {
                case "Function":
                    const [retType, funcName, parameters, funcChildren] = node[x]
                    id += 1
                    treeNode.attrs = {
                        functionName: funcName,
                        returnType: retType,
                        parameters
                    }
                    treeNode.children = node2tree(funcChildren)
                    break
                case "ReturnStatement":
                    treeNode.children = node2tree([node[x]])
                    break
                case "IntLiteral":
                    treeNode.attrs = {
                        value: node[x]
                    }
                    break
                case "Declaration":
                    const [type, name] = node[x]
                    treeNode.attrs = {
                        type,
                        name
                    }
                    break
                case "GlobalDeclaration":
                    const [globalType, globalName, globalAssignment] = node[x]
                    treeNode.attrs = {
                        type: globalType,
                        name: globalName
                    }
                    treeNode.children = node2tree([globalAssignment])
                    break
                case "Assignment":
                    const [assignOperation, leftValue, assignChildren] = node[x]
                    treeNode.attrs = {
                        assignOperation,
                        leftValue
                    }
                    treeNode.children = node2tree([assignChildren])
                    break
                case "Identifier":
                    treeNode.attrs = {
                        name: node[x]
                    }
                    break
                case "BinaryExpression":
                    const [binaryOperator, lhs, rhs] = node[x]
                    treeNode.attrs = {
                        binaryOperator
                    }
                    treeNode.children = node2tree([lhs, rhs])
                    break
                case "ForStatement":
                    const [initClause, condition, iteration, forStatement] = node[x]
                    treeNode.children = node2tree([{InitClause: initClause}, condition, iteration, forStatement])
                    break
                case "InitClause": // InitClause is a special form of BlockStatement
                case "Arguments": // FuncCallArguments is a special form of BlockStatement
                case "BlockStatement":
                    treeNode.children = node2tree(node[x])
                    break
                case "FunctionCall":
                    const [funcCallName, funcArguments] = node[x]
                    treeNode.attrs = {
                        functionName: funcCallName
                    }
                    treeNode.children = node2tree([{Arguments: funcArguments}])
                    break
                case "WhileStatement":
                    const [isDoWhile, whileCondition, whileStatements] = node[x]
                    treeNode.attrs = {isDoWhile}
                    treeNode.children = node2tree([whileCondition, whileStatements])
                    break
                default:
                    console.log(x)
                    console.log(node[x])
            }
            nodeList.push(treeNode)
        }
    }
    return nodeList
}

let id = 0
let tree = {
    id: id.toString(),
    label: "GNC",
    attrs: {
        remark: "AST Root Node"
    },
    children: node2tree(ast)
}

// console.log(JSON.stringify(tree, null, 2))
console.log(JSON.stringify(tree))