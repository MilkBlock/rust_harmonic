use egg::{*, rewrite as rw};



define_language! {
    enum MathObjDescLang {
        "Group" = Group(Vec<Id>),
        "PointConnected" = PointConnected([Id; 2]),
        "SameDirection" = SameDirection([Id; 2]),
        "ArrowRelBundle" = ArrowRelBundele([Id;2]),
        "And" = And([Id;2]),
        "Bundle" = Bundle(Vec<Id>),
        Symbol(Symbol),
    }
}
use egg::{EGraph, Id};


fn group_dfs(egraph: &EGraph<MathObjDescLang, ()>, start: Id) -> Vec<Id> {
    let mut visited = std::collections::HashSet::new();
    let mut to_visit = vec![start];
    let mut connected = Vec::new();

    while let Some(id) = to_visit.pop() {
        if visited.contains(&id) {
            continue;
        }
        visited.insert(id);
        connected.push(id);

        // 获取当前节点的表达式
        for parent in  egraph[id].parents(){
            // 如果是 group 操作，遍历其子节点
            for node in &egraph[parent].nodes{
                if let MathObjDescLang::Group(children) = node{
                    for &child in children {
                        if !visited.contains(&child) {
                            to_visit.push(child);
                        }
                    }
                }
            }
        } 
        
        
        // 也可以添加其他连接逻辑
    }

    connected
}

fn main(){
    let rules = &[
        rw!("commute-Group"; "(Group ?a ?b)" => "(Group ?b ?a)"),
        rw!("commute-PointConnected"; "(PointConnected ?a ?b)" => "(PointConnected ?b ?a)"),
        rw!("commute-SameDirection"; "(SameDirection ?a ?b)" => "(SameDirection ?b ?a)"),
        rw!("commute-And"; "(And ?a ?b)" => "(And ?b ?a)"),
        rw!("ArrowRelBundleRule"; 
            "(And 
                (SameDirection ?a ?b) 
                ( And 
                    (PointConnected ?a ?b) 
                    (Group ?a ?b)
                )
            )" => "(ArrowRelBundle ?a ?b)"),
    ];
    // Rewrite::new(name, searcher, applier);

    let expr0 = 
        "(Group a b)
        ".parse().unwrap();
    let expr1 = 
        "(Group b c)
        ".parse().unwrap();
    // let expr1 = 
    //     "(And
    //         (SameDirection line1 triangle1)
    //         (Group line1 triangle1) 
    //     )
    //     ".parse().unwrap();
    let mut runner: Runner<MathObjDescLang, ()> = Runner::<MathObjDescLang, (), ()>::default().with_explanations_enabled();
    let a = runner.egraph.add(MathObjDescLang::Symbol("a".into()));
    let b = runner.egraph.add(MathObjDescLang::Symbol("b".into()));
    let c = runner.egraph.add(MathObjDescLang::Symbol("c".into()));
    let runner = runner
        .with_expr(&expr0)
        .with_expr(&expr1)
        .run(rules);

    println!("{:?}",group_dfs(&runner.egraph, a));
    // let just_foo = runner.egraph.add_expr(&"foo".parse().unwrap());
    // assert_eq!(runner.egraph.find(runner.roots[0]), runner.egraph.find(just_foo));
    // 生成 DOT 格式
    let dot = runner.egraph.dot();

    // 写入 a.dot 文件
    std::fs::write("a.dot", dot.to_string()).unwrap();

}