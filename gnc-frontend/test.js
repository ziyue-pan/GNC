/***
 * Mocked Data
 */
const data = {
    "error": false, "error_message": "", "parse_tree": {
        "id": "gnc(0,28)", "label": "gnc", "children": [{
            "id": "function(0,28)",
            "label": "function",
            "children": [{
                "id": "data_type(0,3)",
                "label": "data_type",
                "children": [{"id": "int(0,3)", "label": "int", "children": []}]
            }, {"id": "identifier(4,8)", "label": "identifier", "children": []}, {
                "id": "function_parameter_list(8,10)",
                "label": "function_parameter_list",
                "children": []
            }, {
                "id": "statement(17,26)", "label": "statement", "children": [{
                    "id": "return_statement(17,25)", "label": "return_statement", "children": [{
                        "id": "expression(24,25)", "label": "expression", "children": [{
                            "id": "logical_or_expression(24,25)", "label": "logical_or_expression", "children": [{
                                "id": "logical_and_expression(24,25)",
                                "label": "logical_and_expression",
                                "children": [{
                                    "id": "inclusive_or_expression(24,25)",
                                    "label": "inclusive_or_expression",
                                    "children": [{
                                        "id": "exclusive_or_expression(24,25)",
                                        "label": "exclusive_or_expression",
                                        "children": [{
                                            "id": "bitwise_and_expression(24,25)",
                                            "label": "bitwise_and_expression",
                                            "children": [{
                                                "id": "equality_expression(24,25)",
                                                "label": "equality_expression",
                                                "children": [{
                                                    "id": "comparison_expression(24,25)",
                                                    "label": "comparison_expression",
                                                    "children": [{
                                                        "id": "shift_expression(24,25)",
                                                        "label": "shift_expression",
                                                        "children": [{
                                                            "id": "additive_expression(24,25)",
                                                            "label": "additive_expression",
                                                            "children": [{
                                                                "id": "multiplicative_expression(24,25)",
                                                                "label": "multiplicative_expression",
                                                                "children": [{
                                                                    "id": "cast_expression(24,25)",
                                                                    "label": "cast_expression",
                                                                    "children": [{
                                                                        "id": "unary_expression(24,25)",
                                                                        "label": "unary_expression",
                                                                        "children": [{
                                                                            "id": "int_literal(24,25)",
                                                                            "label": "int_literal",
                                                                            "children": [{
                                                                                "id": "dec_literal(24,25)",
                                                                                "label": "dec_literal",
                                                                                "children": []
                                                                            }]
                                                                        }]
                                                                    }]
                                                                }]
                                                            }]
                                                        }]
                                                    }]
                                                }]
                                            }]
                                        }]
                                    }]
                                }]
                            }]
                        }]
                    }]
                }]
            }]
        }, {"id": "EOI(28,28)", "label": "EOI", "children": []}]
    }, "ast": [{"Function": ["Int", "main", [], [{"ReturnStatement": {"IntLiteral": 233}}]]}]
}
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
                        throw new Error(`${x} not implemented`)
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
                        symbolTable[0][funcName] = {
                            retType,
                            parameters,
                            funcBlock: funcChildren
                        }
                        break
                    case "ReturnStatement":
                        res = visitNode([node[x]])
                        // throw new Error(`${x} not implemented`)
                        break
                    case "FunctionCall":
                        const [funcCallName, funcArguments] = node[x]
                        const callee = findSymbol(funcCallName)
                        // put arguments into symbol table
                        symbolTable.push({})
                        // eslint-disable-next-line array-callback-return
                        callee.parameters.map(function(parameter, i) {
                            const [param_type, param_name] = parameter
                            symbolTable[0][param_name] = {
                                type: param_type,
                                value: visitNode(funcArguments[i]) // TODO cast arguments to parameter type
                            }
                        })
                        res = visitNode(callee.funcBlock)
                        // TODO cast to retType
                        res.type = callee.retType
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

const res = evalAST(astData, 'main', []) // {"IntLiteral": 233}
console.log(JSON.stringify(res, null, 2))