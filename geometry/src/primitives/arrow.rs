use algebra_traits::{AddError, SubError, TryAdd, TrySub};


#[derive(Clone, Debug, PartialEq)]
pub struct Arrow<Point> {
    foot:Point,
    head:Point
}

impl<Point> Arrow<Point> {
    pub fn new(foot:Point, head:Point) -> Self {
        Self{foot, head}
    }

    pub fn foot(&self) -> &Point {
        &self.foot
    }

    pub fn head(&self) -> &Point {
        &self.head
    }

    pub fn into_foot_head(self) -> (Point, Point) {
        (self.foot, self.head)
    }

    pub fn revert(self) -> Self {
        -self
    }
}

impl<Point:PartialEq> TryAdd<Point> for Arrow<Point> {
    type Output=Point;
    type Error=AddError;
    fn is_addable_by(&self,rhs:&Point) -> Result<(),Self::Error> {
        if &self.foot == rhs {
            Ok(())
        } else {
            Err(AddError::NotAvailableForProvidedInstances)
        }
    }

    fn try_add(self, rhs:Point) -> Result<Point,AddError> {
        if self.foot == rhs {
            Ok(self.head)
        } else {
            Err(AddError::NotAvailableForProvidedInstances)
        }
    }
}

impl<Point> std::ops::Neg for Arrow<Point> {
    type Output=Self;
    
    fn neg(self) -> Self::Output {
        Self::new(self.head, self.foot)
    }
}

impl<Point:PartialEq> TryAdd for Arrow<Point> {
    type Output=Self;
    type Error=AddError;

    fn is_addable_by(&self,rhs:&Self) -> Result<(),Self::Error> {
        if self.head == rhs.foot {
            Ok(())
        } else {
            Err(AddError::NotAvailableForProvidedInstances)
        }
    }

    fn try_add(self, rhs: Self) -> Result<Arrow<Point>,AddError> {
        if self.head == rhs.foot {
            Ok(Self::new(self.foot, rhs.head))
        } else {
            Err(AddError::NotAvailableForProvidedInstances)
        }
    }
}

impl<Point:PartialEq> TrySub for Arrow<Point> {
    type Output=Self;
    type Error=SubError;

    fn is_subable_by(&self,rhs:&Self) -> Result<(),Self::Error> {
        if self.head == rhs.head {
            Ok(())
        } else {
            Err(SubError::NotAvailableForProvidedInstances)
        }
    }

    fn try_sub(self, rhs: Self) -> Result<Arrow<Point>,SubError> {
        if self.head == rhs.head {
            Ok(Self::new(self.foot, rhs.foot))
        } else {
            Err(SubError::NotAvailableForProvidedInstances)
        }
    }
}

// impl<Point:Clone+Geometric<DIM>, const DIM:usize> Geometric<DIM> for Arrow<Point> {
//     fn apply<T:Transformation<DIM>>(trafo:&T, elem:Self) -> Self {
//         Arrow::new(&trafo.apply(elem.foot().clone()),
//                    &trafo.apply(elem.head().clone()))
//     }
// }