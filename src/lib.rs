use bevy::{prelude::*, utils::HashMap};

#[derive(Component, Default, Eq, Hash, PartialEq, Debug)]
pub struct Cell {
    x: i32,
    y: i32,
}

#[derive(Debug, Default)]
struct CellState {
    entity: Option<Entity>,
    live_neighbors: u32,
}

fn step(mut commands: Commands, live_cells: Query<(Entity, &Cell)>) {
    let mut neighbor_map: HashMap<(i32, i32), CellState> = HashMap::new();

    println!("step");
    for (entity, cell) in live_cells.iter() {
        println!("in: live Cell {:?} at ({}, {})", entity, cell.x, cell.y);

        let state = neighbor_map.entry((cell.x, cell.y)).or_default();
        state.entity = Some(entity);

        for x in cell.x - 1..=cell.x + 1 {
            for y in cell.y - 1..=cell.y + 1 {
                if x == cell.x && y == cell.y {
                    continue;
                }
                let state = neighbor_map.entry((x, y)).or_default();
                state.live_neighbors += 1;
            }
        }
    }

    for ((x, y), state) in neighbor_map.iter() {
        match state {
            CellState {
                entity: None,
                live_neighbors: 3,
            } => {
                println!("out: new Cell at ({}, {})", x, y);
                commands.spawn().insert(Cell { x: *x, y: *y });
            }
            CellState {
                entity: Some(entity),
                live_neighbors: 2 | 3,
            } => println!("out: live Cell {:?} at ({}, {})", entity, x, y),
            CellState {
                entity: Some(entity),
                live_neighbors: _,
            } => {
                println!("out: dead Cell {:?} at ({}, {})", entity, x, y);
                commands.entity(*entity).despawn_recursive();
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_cell_dies() {
        let mut app = App::new();

        app.add_system(step);

        app.world.spawn().insert(Cell { x: 0, y: 0 });

        app.update();

        let entities = app
            .world
            .query::<&Cell>()
            .iter(&app.world)
            .collect::<Vec<_>>();

        assert!(entities.is_empty());
    }

    #[test]
    fn two_cells_die() {
        let mut app = App::new();

        app.add_system(step);

        app.world.spawn().insert(Cell { x: 0, y: 0 });
        app.world.spawn().insert(Cell { x: 1, y: 0 });

        app.update();

        let entities = app
            .world
            .query::<&Cell>()
            .iter(&app.world)
            .collect::<Vec<_>>();

        assert!(entities.is_empty());
    }

    #[test]
    fn blinker_lives() {
        let mut app = App::new();

        app.add_system(step);

        app.world.spawn().insert(Cell { x: -1, y: 0 });
        app.world.spawn().insert(Cell { x: 0, y: 0 });
        app.world.spawn().insert(Cell { x: 1, y: 0 });

        app.update();

        let mut entities = app
            .world
            .query::<&Cell>()
            .iter(&app.world)
            .map(|cell| (cell.x, cell.y))
            .collect::<Vec<_>>();

        entities.sort();
        assert_eq!(entities, vec![(0, -1), (0, 0), (0, 1),]);

        app.update();

        let mut entities = app
            .world
            .query::<&Cell>()
            .iter(&app.world)
            .map(|cell| (cell.x, cell.y))
            .collect::<Vec<_>>();

        entities.sort();
        assert_eq!(entities, vec![(-1, 0), (0, 0), (1, 0),]);
    }

    #[test]
    fn block_lives() {
        let mut app = App::new();

        app.add_system(step);

        app.world.spawn().insert(Cell { x: 0, y: 0 });
        app.world.spawn().insert(Cell { x: 0, y: 1 });
        app.world.spawn().insert(Cell { x: 1, y: 0 });
        app.world.spawn().insert(Cell { x: 1, y: 1 });

        app.update();

        let mut entities = app
            .world
            .query::<&Cell>()
            .iter(&app.world)
            .map(|cell| (cell.x, cell.y))
            .collect::<Vec<_>>();

        entities.sort();
        assert_eq!(entities, vec![(0, 0), (0, 1), (1, 0), (1, 1)]);

        app.update();

        let mut entities = app
            .world
            .query::<&Cell>()
            .iter(&app.world)
            .map(|cell| (cell.x, cell.y))
            .collect::<Vec<_>>();

        entities.sort();
        assert_eq!(entities, vec![(0, 0), (0, 1), (1, 0), (1, 1)]);
    }
}
