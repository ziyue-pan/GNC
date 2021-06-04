/***
 * Mocked Data
 */
const data = {"error":false,"error_message":"","parse_tree":{"id":"gnc(0,33)","label":"gnc","children":[{"id":"function(0,33)","label":"function","children":[{"id":"data_type(0,3)","label":"data_type","children":[{"id":"int(0,3)","label":"int","children":[]}]},{"id":"identifier(4,8)","label":"identifier","children":[]},{"id":"function_parameter_list(8,15)","label":"function_parameter_list","children":[{"id":"function_parameter(9,14)","label":"function_parameter","children":[{"id":"data_type(9,12)","label":"data_type","children":[{"id":"int(9,12)","label":"int","children":[]}]},{"id":"identifier(13,14)","label":"identifier","children":[]}]}]},{"id":"statement(22,31)","label":"statement","children":[{"id":"return_statement(22,30)","label":"return_statement","children":[{"id":"expression(29,30)","label":"expression","children":[{"id":"logical_or_expression(29,30)","label":"logical_or_expression","children":[{"id":"logical_and_expression(29,30)","label":"logical_and_expression","children":[{"id":"inclusive_or_expression(29,30)","label":"inclusive_or_expression","children":[{"id":"exclusive_or_expression(29,30)","label":"exclusive_or_expression","children":[{"id":"bitwise_and_expression(29,30)","label":"bitwise_and_expression","children":[{"id":"equality_expression(29,30)","label":"equality_expression","children":[{"id":"comparison_expression(29,30)","label":"comparison_expression","children":[{"id":"shift_expression(29,30)","label":"shift_expression","children":[{"id":"additive_expression(29,30)","label":"additive_expression","children":[{"id":"multiplicative_expression(29,30)","label":"multiplicative_expression","children":[{"id":"cast_expression(29,30)","label":"cast_expression","children":[{"id":"unary_expression(29,30)","label":"unary_expression","children":[{"id":"identifier(29,30)","label":"identifier","children":[]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]},{"id":"EOI(33,33)","label":"EOI","children":[]}]},"ast":[{"Function":["Int","main",[{"param_type":"Int","param_name":"a"}],[{"ReturnStatement":{"Identifier":"a"}}]]}]}
const astData = data.ast

/***
 * HLVM - High Level Virtual Machine
 * @param ast AST to be interpreted
 * @param entryPoint entry function name
 * @param args arguments as AST, e.g. {"IntLiteral": 233}
 * @returns {type, value} result
 */

function evalAST(ast, entryPoint, args) {
    // the entering & leaving of scopes will be maintained by the visitor function
    let symbolTable = [{}]

    const findSymbol = (symbolName) => {
        // iterate from top of the stack to emulate scope's name hiding
        for (const scopeTable of symbolTable.reverse()) {
            if (symbolName in scopeTable) {
                return scopeTable[symbolName] // return ref of the symbol
            }
        }
        throw Error(`Can not find symbol ${symbolName}`)
    }

    const castValue = (origData, destType) => {
        let res = {
            type: destType,
            value: origData.value
        }
        switch (destType) {
            case "Int":
                res.value = Math.floor(origData.value)
                break
            default:
                throw new Error(`Cast to ${destType} not implemented`)
        }
        return res
    }

    const visitNode = (astNode) => {
        let res = null
        for (const node of astNode) {
            if (node === "BreakStatement" || node === "ContinueStatement") { // special statements
                console.log(node)
                throw new Error(node)
            } else for (const x in node) { // Actually there's only one entry inside
                switch (x) {
                    // basics
                    case "Assignment":
                        const [assignOperation, leftValue, assignChildren] = node[x]
                        throw new Error(`${x} not implemented`)
                        break
                    case "Identifier":
                        res = findSymbol(node[x])
                        break

                    // literals
                    case "IntLiteral":
                        res = {
                            type: "Int",
                            value: node[x]
                        }
                        break
                    case "BoolLiteral":
                        throw new Error(`${x} not implemented`)
                        break
                    case "FloatLiteral":
                        throw new Error(`${x} not implemented`)
                        break

                    // declarations
                    case "Declaration":
                        const [type, name] = node[x]
                        throw new Error(`${x} not implemented`)
                        break
                    case "GlobalDeclaration":
                        const [globalType, globalName, globalAssignment] = node[x]
                        throw new Error(`${x} not implemented`)
                        break

                    // Expressions
                    case "BinaryExpression":
                        const [binaryOperator, lhs, rhs] = node[x]
                        throw new Error(`${x} not implemented`)
                        break
                    case "UnaryExpression":
                        const [unaryOperator, unaryExpression] = node[x]
                        throw new Error(`${x} not implemented`)
                        break
                    case "CastExpression":
                        const [castType, castExpression] = node[x]
                        throw new Error(`${x} not implemented`)
                        break

                    // functions
                    case "Function":
                        const [retType, funcName, parameters, funcChildren] = node[x]
                        symbolTable.slice(-1)[0][funcName] = {
                            retType,
                            parameters,
                            funcBlock: funcChildren
                        }
                        break
                    case "ReturnStatement":
                        res = visitNode([node[x]])
                        break
                    case "FunctionCall":
                        const [funcCallName, funcArguments] = node[x]
                        const callee = findSymbol(funcCallName)
                        // put arguments into symbol table
                        symbolTable.push({})
                        // eslint-disable-next-line array-callback-return
                        callee.parameters.map(function(parameter, i) {
                            const {param_type, param_name} = parameter
                            symbolTable.slice(-1)[0][param_name] = castValue(visitNode([funcArguments[i]]), param_type)
                        })
                        res = castValue(visitNode(callee.funcBlock), callee.retType)
                        symbolTable.pop()
                        break

                    // flow controls & statements
                    case "ForStatement":
                        const [initClause, condition, iteration, forStatement] = node[x]
                        throw new Error(`${x} not implemented`)
                        break
                    case "WhileStatement":
                        const [isDoWhile, whileCondition, whileStatements] = node[x]
                        throw new Error(`${x} not implemented`)
                        break
                    case "IfStatement": // IfStatements is visualized as a list of BlockStatements
                    case "InitClause": // InitClause is a special form of BlockStatement
                    case "Arguments": // FuncCallArguments is a special form of BlockStatements
                    case "BlockStatement":
                        throw new Error(`${x} not implemented`)
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
        }
        return res
    }

    // scan global
    visitNode(ast)

    // call entry function
    const funcCall = [{
        "FunctionCall": [
            entryPoint,
            args
        ]
    }]
    const res = visitNode(funcCall)

    return res
}

const res = evalAST(astData, 'main', [{"IntLiteral": 666.233333}])
console.log(JSON.stringify(res, null, 2))