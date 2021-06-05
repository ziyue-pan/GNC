export default function AST2VisualizationData(ast) {
    let id = 0
    const node2tree = (astNode) => {
        let nodeList = []
        for (const node of astNode) {
            id += 1
            let treeNode = {
                id: id.toString(),
                label: "",
                attrs: {},
                children: []
            }
            if (node === "BreakStatement" || node === "ContinueStatement") { // special statements
                treeNode.label = node
            } else for (const x in node) { // Actually there's only one entry inside
                treeNode.label = x
                switch (x) {
                    // basics
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

                    // literals
                    case "IntLiteral":
                    case "BoolLiteral":
                    case "FloatLiteral":
                        treeNode.attrs = {
                            value: node[x]
                        }
                        break

                    // declarations
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

                    // Expressions
                    case "BinaryExpression":
                        const [binaryOperator, lhs, rhs] = node[x]
                        treeNode.attrs = {
                            binaryOperator
                        }
                        treeNode.children = node2tree([lhs, rhs])
                        break
                    case "UnaryExpression":
                        const [unaryOperator, unaryExpression] = node[x]
                        treeNode.attrs = {
                            unaryOperator
                        }
                        treeNode.children = node2tree([unaryExpression])
                        break
                    case "CastExpression":
                        const [castType, castExpression] = node[x]
                        treeNode.attrs = {
                            castType
                        }
                        treeNode.children = node2tree([castExpression])
                        break

                    // functions
                    case "Function":
                        const [retType, funcName, parameters, funcChildren] = node[x]
                        id += 1
                        treeNode.attrs = {
                            functionName: funcName,
                            returnType: retType,
                            parameters: JSON.stringify(parameters, null, 2)
                        }
                        treeNode.children = node2tree(funcChildren)
                        break
                    case "ReturnStatement":
                        treeNode.children = node2tree([node[x]])
                        break
                    case "FunctionCall":
                        const [funcCallName, funcArguments] = node[x]
                        treeNode.attrs = {
                            functionName: funcCallName
                        }
                        treeNode.children = node2tree([{Arguments: funcArguments}])
                        break

                    // flow controls & statements
                    case "ForStatement":
                        const [initClause, condition, iteration, forStatement] = node[x]
                        treeNode.children = node2tree([{InitClause: initClause}, condition, iteration, forStatement])
                        break
                    case "WhileStatement":
                        const [isDoWhile, whileCondition, whileStatements] = node[x]
                        treeNode.attrs = {isDoWhile}
                        treeNode.children = node2tree([whileCondition, whileStatements])
                        break
                    case "IfStatement": // IfStatements is visualized as a list of BlockStatements
                    case "InitClause": // InitClause is a special form of BlockStatement
                    case "Arguments": // FuncCallArguments is a special form of BlockStatements
                    case "BlockStatement":
                        treeNode.children = node2tree(node[x])
                        break
                    case "BreakStatement":
                    case "ContinueStatement":
                        break

                    // unhandled case
                    default:
                        console.log(x)
                        console.log(node[x])
                }
            }
            nodeList.push(treeNode)
        }
        return nodeList
    }
    let tree = {
        id: id.toString(),
        label: "GNC",
        attrs: {
            remark: "AST Root Node"
        },
        children: node2tree(ast)
    }
    return tree
}