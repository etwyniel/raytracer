use std::str::FromStr;
use super::{Vec3, Solid};

pub struct Sphere {
    pub center: Vec3<f64>,
    pub radius: f64,
    pub radius2: f64
}

impl Sphere {
    pub fn new(center: Vec3<f64>, radius: f64) -> Self {
        Sphere {center, radius, radius2: radius * radius}
    }
}

impl Solid for Sphere {
    fn intersect(&self, org: Vec3<f64>, dir: Vec3<f64>) -> Option<f64> {
        let l = self.center - org;
        let tca = l.dot(&dir);
        if tca < 0. {return None;};
        let d2 = l.dot(&l) - tca * tca;
        if d2 > self.radius2 {return None;};
        let thc = (self.radius2 - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
        if t0 > t1 {
            if t1 < 0. {if t0 < 0. {None} else {Some(t0)}} else {Some(t1)}
        } else {
            if t0 < 0. {if t1 < 0. {None} else {Some(t1)}} else{Some(t0)}
        }
        /*
        let l = org - self.center;
        match solve_quadratic(&Vec3::new(dir.dot(&dir), 2. * dir.dot(&l),
            l.dot(&l) - self.radius2)) {

            None => None,
            Some((t0, t1)) => {
                let (t0, t1) = if t0 > t1 {(t1, t0)} else {(t0, t1)};
                if t0 < 0. {
                    if t1 < 0. {None} else {Some(t1)}
                } else {
                    Some(t0)
                }
            }
        }
        */
    }
    /*
    fn emission_color(&self) -> Vec3<f64> {self.emission_color}
    fn surface_color(&self) -> Vec3<f64> {self.surface_color}
    fn transparency(&self) -> f64 {self.transparency}
    fn reflection(&self) -> f64 {self.reflection}
    */
    fn position(&self) -> Vec3<f64> {self.center}

    fn normal_at(&self, hit: Vec3<f64>, _dir: Vec3<f64>) -> Vec3<f64> {
        let mut res = hit - self.center;
        res.normalize();
        res
    }
}

impl FromStr for Sphere {
    type Err = ();
    fn from_str(s:  &str) -> Result<Self, Self::Err> {
        let ind = s.rfind(' ').ok_or(())?;
        let (center_str, radius_str) = s.split_at(ind);
        let center = Vec3::from_str(center_str).or_else(|_| Err(()))?;
        let radius = f64::from_str(radius_str.trim()).or_else(|_| Err(()))?;
        Ok(Self::new(center, radius))
    }
}
