use std::fmt::Formatter;
use std::fmt::Error;

// Body struct
struct Body {
    x: f32,
    y: f32,
    v_x: f32, 
    v_y: f32,
    f_x: f32,
    f_y: f32,
    m: f32,
}

impl Body {
    fn add_force(&self, body: Body) {
        let dx = self.x - body.x;
        let dy = self.y - body.y;
        let dist = self.distance_from(body);

        let force_mag = crate::G * self.m * body.m / (dist.powi(2) + crate::S.powi(2));
        self.f_x += force_mag*dx / dist;
        self.f_y += force_mag*dy / dist;
    }


    fn reset_force(&self) {
        self.f_x = 0.0;
        self.f_y = 0.0;
    }


    fn update_velocity(&self, dt: f32) {
        self.v_x = self.v_x + self.m*self.f_x*dt;
        self.v_y = self.v_y + self.m*self.f_y*dt;
    }


    fn update_position(&self, dt: f32) {
        self.x = self.x + self.v_x*dt;
        self.y = self.y + self.v_y*dt;
    }


    fn distance_from(&self, body: Body) -> f32 {
        let dx = self.x - body.x;
        let dy = self.y - body.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

struct System {
    dt: f32, 
    bodies: [Body],
}

impl System {
    fn create_system() -> System {
        // Construct from file

    }

    fn initialize_system(&self) {
        self.update_forces();
    }

    fn update_forces(&self) {
        for i in 1..self.bodies.len() {
            for j in 1..self.bodies.len() {
                if i != j {
                    &self.bodies[i].add_force(&self.bodies[j]);
                }
            }
        }
    }

    fn reset_forces(&self) {
        for i in 1..self.bodies.len() {
            &self.bodies[i].reset_force();
        }
    }

    fn update_system(&self) {
        for &body in self.bodies {
            body.update_velocity(0.5*self.dt);
            body.update_position(self.dt);
        }
        self.reset_forces();
        self.update_forces();
        for &body in self.bodies {
            body.update_velocity(0.5*self.dt);
        }
    }
}

impl std::fmt::Display for System {
    
}

impl std::fmt::Display for Body {

}

#[cfg(test)]
mod tests {
    #[test]
    
}
