All binary operations are LEFT associative this is so that ambigous expressions like:
a -> b -> c will always be parsed into (a -> b) -> c


The functions which this will animate are:


procedure obdd (F)
input: propositional formula F
parameters: global dag D
output: a node n in (modified) D which represents F
begin
    F := simplify(F)
    if F = ⊥ then return 0
    if F = ⊤ then return 1
    p := max_variable(F)
    n1 := obdd($F^{⊥}_{p}$)
    n2 := obdd($F^{⊤}_{p}$)
    return integrate(n1, p, n2, D)
end


procedure integrate(n1, p, n2, D)
parameters: global dag D
input: nodes n1, n2 in D representing formulas F1, F2, variable p
output: node n in (modified) D representing if p then F1 else F2
begin
    if n1 = n2 then return n1;
    if D contains a node n having the form p
                                          / \ 
                                        n1  n2
    then return n;
    add to D a new node n of the form p ;
                                     / \
                                    n1  n2
    return n
end
