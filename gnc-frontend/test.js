/***
 * Mocked Data
 */
const data = {"error":false,"error_message":"","parse_tree":{"id":"gnc(0,83)","label":"gnc","children":[{"id":"function(0,39)","label":"function","children":[{"id":"data_type(0,4)","label":"data_type","children":[{"id":"int(0,3)","label":"int","children":[]}]},{"id":"identifier(4,8)","label":"identifier","children":[]},{"id":"function_parameter_list(8,10)","label":"function_parameter_list","children":[]},{"id":"statement(17,23)","label":"statement","children":[{"id":"expression(17,22)","label":"expression","children":[{"id":"assignment_expression(17,22)","label":"assignment_expression","children":[{"id":"unary_expression(17,18)","label":"unary_expression","children":[{"id":"identifier(17,18)","label":"identifier","children":[]}]},{"id":"assign_simple(19,20)","label":"assign_simple","children":[]},{"id":"expression(21,22)","label":"expression","children":[{"id":"logical_or_expression(21,22)","label":"logical_or_expression","children":[{"id":"logical_and_expression(21,22)","label":"logical_and_expression","children":[{"id":"inclusive_or_expression(21,22)","label":"inclusive_or_expression","children":[{"id":"exclusive_or_expression(21,22)","label":"exclusive_or_expression","children":[{"id":"bitwise_and_expression(21,22)","label":"bitwise_and_expression","children":[{"id":"equality_expression(21,22)","label":"equality_expression","children":[{"id":"comparison_expression(21,22)","label":"comparison_expression","children":[{"id":"shift_expression(21,22)","label":"shift_expression","children":[{"id":"additive_expression(21,22)","label":"additive_expression","children":[{"id":"multiplicative_expression(21,22)","label":"multiplicative_expression","children":[{"id":"cast_expression(21,22)","label":"cast_expression","children":[{"id":"unary_expression(21,22)","label":"unary_expression","children":[{"id":"int_literal(21,22)","label":"int_literal","children":[{"id":"dec_literal(21,22)","label":"dec_literal","children":[]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]},{"id":"statement(28,37)","label":"statement","children":[{"id":"return_statement(28,36)","label":"return_statement","children":[{"id":"expression(35,36)","label":"expression","children":[{"id":"logical_or_expression(35,36)","label":"logical_or_expression","children":[{"id":"logical_and_expression(35,36)","label":"logical_and_expression","children":[{"id":"inclusive_or_expression(35,36)","label":"inclusive_or_expression","children":[{"id":"exclusive_or_expression(35,36)","label":"exclusive_or_expression","children":[{"id":"bitwise_and_expression(35,36)","label":"bitwise_and_expression","children":[{"id":"equality_expression(35,36)","label":"equality_expression","children":[{"id":"comparison_expression(35,36)","label":"comparison_expression","children":[{"id":"shift_expression(35,36)","label":"shift_expression","children":[{"id":"additive_expression(35,36)","label":"additive_expression","children":[{"id":"multiplicative_expression(35,36)","label":"multiplicative_expression","children":[{"id":"cast_expression(35,36)","label":"cast_expression","children":[{"id":"unary_expression(35,36)","label":"unary_expression","children":[{"id":"identifier(35,36)","label":"identifier","children":[]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]},{"id":"global_variable(42,53)","label":"global_variable","children":[{"id":"data_type(42,46)","label":"data_type","children":[{"id":"int(42,45)","label":"int","children":[]}]},{"id":"identifier(46,47)","label":"identifier","children":[]},{"id":"expression(51,52)","label":"expression","children":[{"id":"logical_or_expression(51,52)","label":"logical_or_expression","children":[{"id":"logical_and_expression(51,52)","label":"logical_and_expression","children":[{"id":"inclusive_or_expression(51,52)","label":"inclusive_or_expression","children":[{"id":"exclusive_or_expression(51,52)","label":"exclusive_or_expression","children":[{"id":"bitwise_and_expression(51,52)","label":"bitwise_and_expression","children":[{"id":"equality_expression(51,52)","label":"equality_expression","children":[{"id":"comparison_expression(51,52)","label":"comparison_expression","children":[{"id":"shift_expression(51,52)","label":"shift_expression","children":[{"id":"additive_expression(51,52)","label":"additive_expression","children":[{"id":"multiplicative_expression(51,52)","label":"multiplicative_expression","children":[{"id":"cast_expression(51,52)","label":"cast_expression","children":[{"id":"unary_expression(51,52)","label":"unary_expression","children":[{"id":"int_literal(51,52)","label":"int_literal","children":[{"id":"dec_literal(51,52)","label":"dec_literal","children":[]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]},{"id":"function(55,82)","label":"function","children":[{"id":"data_type(55,59)","label":"data_type","children":[{"id":"int(55,58)","label":"int","children":[]}]},{"id":"identifier(59,62)","label":"identifier","children":[]},{"id":"function_parameter_list(62,64)","label":"function_parameter_list","children":[]},{"id":"statement(71,80)","label":"statement","children":[{"id":"return_statement(71,79)","label":"return_statement","children":[{"id":"expression(78,79)","label":"expression","children":[{"id":"logical_or_expression(78,79)","label":"logical_or_expression","children":[{"id":"logical_and_expression(78,79)","label":"logical_and_expression","children":[{"id":"inclusive_or_expression(78,79)","label":"inclusive_or_expression","children":[{"id":"exclusive_or_expression(78,79)","label":"exclusive_or_expression","children":[{"id":"bitwise_and_expression(78,79)","label":"bitwise_and_expression","children":[{"id":"equality_expression(78,79)","label":"equality_expression","children":[{"id":"comparison_expression(78,79)","label":"comparison_expression","children":[{"id":"shift_expression(78,79)","label":"shift_expression","children":[{"id":"additive_expression(78,79)","label":"additive_expression","children":[{"id":"multiplicative_expression(78,79)","label":"multiplicative_expression","children":[{"id":"cast_expression(78,79)","label":"cast_expression","children":[{"id":"unary_expression(78,79)","label":"unary_expression","children":[{"id":"int_literal(78,79)","label":"int_literal","children":[{"id":"dec_literal(78,79)","label":"dec_literal","children":[]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]},{"id":"EOI(83,83)","label":"EOI","children":[]}]},"ast":[{"Function":["Int","main",[],[{"Assignment":["Simple",{"Identifier":"a"},{"IntLiteral":3}]},{"ReturnStatement":{"Identifier":"a"}}]]},{"GlobalDeclaration":["Int","a",{"IntLiteral":0}]},{"Function":["Int","foo",[],[{"ReturnStatement":{"IntLiteral":0}}]]}]}
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
    let callStack = []

    const findSymbol = (symbolName) => {
        // iterate from top of the stack to emulate scope's name hiding
        for (const scopeTable of symbolTable.slice().reverse()) {
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
            // [Control Flow] if function has returned, stop execution
            if (callStack.length && callStack.slice(-1)[0].hasReturn) {
                return res
            }
            if (node === "BreakStatement" || node === "ContinueStatement") { // special statements
                switch (node) {
                    case "BreakStatement":
                        const loopStack = callStack.slice(-1)[0].loopStack
                        loopStack[loopStack.length - 1] = true
                        return res // [Control Flow] stop exec following blocks after break
                    case "ContinueStatement":
                        throw new Error(`${node} not implemented`)
                    default:
                        throw new Error('strange thing')
                }
            } else for (const x in node) { // Actually there's only one entry inside
                switch (x) {
                    // basics
                    case "Assignment":
                        const [assignOperation, leftValue, assignChildren] = node[x]
                        const variable = visitNode([leftValue]) // TODO implement real leftValue
                        res = castValue(visitNode([assignChildren]), variable.type)
                        switch (assignOperation) {
                            case "Simple":
                                variable.value = res.value
                                break
                            case "Addition":
                                variable.value += res.value
                                break
                            case "Subtraction":
                                variable.value -= res.value
                                break
                            case "Multiplication":
                                variable.value *= res.value
                                break
                            case "Division":
                                variable.value /= res.value
                                break
                            case "Modulus":
                                variable.value %= res.value
                                break
                            case "BitwiseAnd":
                                variable.value &= res.value
                                break
                            case "InclusiveOr":
                                variable.value |= res.value
                                break
                            case "ExclusiveOr":
                                variable.value ^= res.value
                                break
                            case "ShiftLeft":
                                variable.value <<= res.value
                                break
                            case "ShiftRight":
                                variable.value >>= res.value
                                break
                            default:
                                throw new Error(`${assignOperation} not implemented`)
                        }
                        res = variable
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
                        symbolTable.slice(-1)[0][name] = {
                            type,
                            value: 0
                        }
                        break
                    case "GlobalDeclaration":
                        const [globalType, globalName, globalAssignment] = node[x]
                        symbolTable.slice(-1)[0][globalName] = {
                            type: globalType,
                            value: castValue(visitNode([globalAssignment]).value, globalType)
                        }
                        break

                    // Expressions
                    case "BinaryExpression":
                        const [binaryOperator, lhs, rhs] = node[x]
                        const leftRes = visitNode([lhs])
                        const rightRes = visitNode([rhs])
                        switch (binaryOperator) {
                            case "Add":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value + rightRes.value
                                }
                                break
                            case "Subtract":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value - rightRes.value
                                }
                                break
                            case "Multiply":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value * rightRes.value
                                }
                                break
                            case "Modulus":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value % rightRes.value
                                }
                                break
                            case "Divide":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value / rightRes.value
                                }
                                break
                            case "Equal":
                                res = {
                                    type: 'Bool',
                                    value: leftRes.value === rightRes.value
                                }
                                break
                            case "ShiftRight":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value >> rightRes.value
                                }
                                break
                            case "ShiftLeft":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value << rightRes.value
                                }
                                break
                            case "LessThan":
                                res = {
                                    type: 'Bool',
                                    value: leftRes.value < rightRes.value
                                }
                                break
                            case "GreaterThan":
                                res = {
                                    type: 'Bool',
                                    value: leftRes.value > rightRes.value
                                }
                                break
                            case "LessEqual":
                                res = {
                                    type: 'Bool',
                                    value: leftRes.value <= rightRes.value
                                }
                                break
                            case "GreaterEqual":
                                res = {
                                    type: 'Bool',
                                    value: leftRes.value >= rightRes.value
                                }
                                break
                            case "NotEqual":
                                res = {
                                    type: 'Bool',
                                    value: leftRes.value !== rightRes.value
                                }
                                break
                            case "BitwiseAnd":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value & rightRes.value
                                }
                                break
                            case "ExclusiveOr":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value ^ rightRes.value
                                }
                                break
                            case "InclusiveOr":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value | rightRes.value
                                }
                                break
                            case "LogicalAnd":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value && rightRes.value
                                }
                                break
                            case "LogicalOr":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value || rightRes.value
                                }
                                break
                            case "FetchRHS":
                                throw new Error(`FetchRHS should not appear at frontend`)
                            default:
                                throw new Error(`${binaryOperator} not implemented`)
                        }
                        break
                    case "UnaryExpression":
                        const [unaryOperator, unaryExpression] = node[x]
                        res = visitNode([unaryExpression])
                        switch (unaryOperator) {
                            case "UnaryMinus":
                                res.value *= -1
                                break
                            case "BitwiseComplement":
                                res.value = ~res.value
                                break
                            case "LogicalNot":
                                res.value = !res.value
                                break
                            default:
                                throw new Error(`${unaryOperator} not implemented`)
                        }
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
                        callStack.slice(-1)[0].returnResult = res
                        callStack.slice(-1)[0].hasReturn = true
                        return res // [!] break current exec flow
                    case "FunctionCall":
                        const [funcCallName, funcArguments] = node[x]
                        const callee = findSymbol(funcCallName)
                        // put arguments into symbol table
                        symbolTable.push({})
                        callStack.push({
                            funcCallName,
                            funcArguments,
                            returnResult: null,
                            hasReturn: false,
                            loopStack: []
                        })
                        // eslint-disable-next-line array-callback-return
                        callee.parameters.map(function(parameter, i) {
                            const {param_type, param_name} = parameter
                            symbolTable.slice(-1)[0][param_name] = castValue(visitNode([funcArguments[i]]), param_type)
                        })
                        visitNode(callee.funcBlock)
                        res = castValue(callStack.slice(-1)[0].returnResult, callee.retType)
                        callStack.pop()
                        symbolTable.pop()
                        break

                    // flow controls & statements
                    case "ForStatement":
                        const [initClause, condition, iteration, forStatement] = node[x]
                        const forLoopStack = callStack.slice(-1)[0].loopStack
                        forLoopStack.push(false)
                        for (visitNode(initClause); !forLoopStack.slice(-1)[0] && visitNode([condition]).value; visitNode([iteration])) {
                            visitNode([forStatement])
                        }
                        forLoopStack.pop()
                        break
                    case "WhileStatement":
                        const whileLoopStack = callStack.slice(-1)[0].loopStack
                        const [isDoWhile, whileCondition, whileStatements] = node[x]
                        whileLoopStack.push(false)
                        if (isDoWhile) {
                            do {
                                visitNode([whileStatements])
                            } while (!whileLoopStack.slice(-1)[0] && visitNode([whileCondition]).value)
                        } else {
                            while (!whileLoopStack.slice(-1)[0] && visitNode([whileCondition]).value) {
                                visitNode([whileStatements])
                            }
                        }
                        whileLoopStack.pop()
                        break
                    case "IfStatement":
                        const [ifCondExpression, ifTrueStatement, elseStatement] = node[x]
                        const ifRes = visitNode([ifCondExpression])
                        if (ifRes.value) {
                            visitNode([ifTrueStatement])
                        } else {
                            visitNode([elseStatement])
                        }
                        break
                    case "InitClause": // InitClause is a special form of BlockStatement
                    case "Arguments": // FuncCallArguments is a special form of BlockStatements
                    case "BlockStatement":
                        visitNode(node[x])
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

    // scan global symbols
    visitNode(ast)

    // call entry function
    const funcCall = [{
        "FunctionCall": [
            entryPoint,
            args
        ]
    }]
    return visitNode(funcCall)
}

const res = evalAST(astData, 'main', [])
console.log(JSON.stringify(res, null, 2))