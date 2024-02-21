use draw3d::app::App;

mod tree;

use tree::Tree;

fn main() {
    draw3d::app(model)
        .view(view)
        .update(update)
        .run();
}

struct Model {
    tree: Tree,
}

fn model(_app: &App) -> Model {
    let tree = Tree::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    
    Model {
        tree
    }
}

fn view(app: &mut App, model: &Model) {
    let mut draw = app.draw();
    draw.add(&model.tree, app);

    app.draw_to_frame(draw);
}

fn update(_: &App, model: &mut Model) {
    
}