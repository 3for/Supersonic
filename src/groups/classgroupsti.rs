
use crate::util::{TypeRep};
use super::{Group, UnknownOrderGroup, ElemFrom, HashPrime};
use rug::Integer;

pub use classygroup::{
    ClassElem,
    ClassGroup as ClassyGroup,
    group::CLASS_GROUP_DISCRIMINANT,
    num::Mpz,
    hash,
};


impl TypeRep for ClassyGroup {
  type Rep = Mpz;
  fn rep() -> &'static Self::Rep {
    &CLASS_GROUP_DISCRIMINANT
  }
}

impl Group for ClassyGroup {
  type Elem = ClassElem;

  #[allow(non_snake_case)]
  fn op_(_: &Mpz, x: &ClassElem, y: &ClassElem) -> ClassElem {
    ClassyGroup::op(x, y)
  }

  // Constructs the reduced element directly instead of using `Self::Elem()`.
  fn id_(_: &Mpz) -> ClassElem {
    return ClassyGroup::id();
  }

  // Constructs the inverse directly instead of using `Self::Elem()`.
  fn inv_(_: &Mpz, x: &ClassElem) -> ClassElem {
    ClassyGroup::inv(x)
  }

  fn exp_(_: &Mpz, a: &ClassElem, n: &Integer) -> Option<ClassElem> {
    Some(ClassyGroup::pow(a, n))
  }
}

impl UnknownOrderGroup for ClassyGroup {
  fn unknown_order_elem_(_: &Mpz) -> ClassElem {
    ClassyGroup::unknown_order_elem()
  }
}

/// Panics if `(a, b, c)` cannot be reduced to a valid class element.
impl<A, B, C> ElemFrom<(A, B, C)> for ClassyGroup
where
  Mpz: From<A>,
  Mpz: From<B>,
  Mpz: From<C>,
{
  fn elemnew(abc: (A, B, C)) -> ClassElem {
    ClassyGroup::elem((Mpz::from(abc.0), Mpz::from(abc.1), Mpz::from(abc.2)))
  }
}

impl HashPrime for ClassyGroup {
    fn pick_prime_mpz(t: &[u8]) -> Mpz {
        hash::hash_to_prime_Mpz(t)
    }

    fn pick_prime_Integer(t: &[u8]) -> Integer {
        hash::hash_to_prime(t)
    }
}


//  Caveat: tests that use "ground truth" use outputs from
//  Chia's sample implementation in python:
//    https://github.com/Chia-Network/vdf-competition/blob/master/inkfish/classgroup.py.
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    // Makes a class elem tuple but does not reduce.
    fn construct_raw_elem_from_strings(a: &str, b: &str, c: &str) -> ClassElem {
        ClassElem {
            a: Mpz::from_str(a).unwrap(),
            b: Mpz::from_str(b).unwrap(),
            c: Mpz::from_str(c).unwrap(),
        }
    }

    #[should_panic]
    #[test]
    fn test_bad_elem() {
        let _ = ClassyGroup::elemnew((Mpz::from(1), Mpz::from(2), Mpz::from(3)));
    }

    #[test]
    fn test_elem_from() {
        let a1 = Mpz::from_str("16").unwrap();
        let b1 = Mpz::from_str("105").unwrap();
        let c1 = Mpz::from_str(
      "47837607866886756167333839869251273774207619337757918597995294777816250058331116325341018110\
      672047217112377476473502060121352842575308793237621563947157630098485131517401073775191194319\
      531549483898334742144138601661120476425524333273122132151927833887323969998955713328783526854\
      198871332313399489386997681827578317938792170918711794684859311697439726596656501594138449739\
      494228617068329664776714484742276158090583495714649193839084110987149118615158361352488488402\
      038894799695420483272708933239751363849397287571692736881031223140446926522431859701738994562\
      9057462766047140854869124473221137588347335081555186814207",
    )
    .unwrap();

        let a2 = Mpz::from_str("16").unwrap();
        let b2 = Mpz::from_str("9").unwrap();
        let c2 = Mpz::from_str(
      "47837607866886756167333839869251273774207619337757918597995294777816250058331116325341018110\
      672047217112377476473502060121352842575308793237621563947157630098485131517401073775191194319\
      531549483898334742144138601661120476425524333273122132151927833887323969998955713328783526854\
      198871332313399489386997681827578317938792170918711794684859311697439726596656501594138449739\
      494228617068329664776714484742276158090583495714649193839084110987149118615158361352488488402\
      038894799695420483272708933239751363849397287571692736881031223140446926522431859701738994562\
      9057462766047140854869124473221137588347335081555186814036",
    )
    .unwrap();

        let reduced_elem = ClassyGroup::elemnew((a1, b1, c1));
        let also_reduced_elem = ClassyGroup::elemnew((a2, b2, c2));
        assert_eq!(reduced_elem, also_reduced_elem);
    }

    #[test]
    fn test_op_single() {
        let a = construct_raw_elem_from_strings(
      "4",
      "1",
      "19135043146754702466933535947700509509683047735103167439198117911126500023332446530136407244\
      268818886844950990589400824048541137030123517295048625578863052039394052606960429510076477727\
      812619793559333896857655440664448190570209733309248852860771133554929587999582285331513410741\
      679548532925359795754799072731031327175516868367484717873943724678975890638662600637655379895\
      797691446827331865910685793896910463236233398285859677535633644394859647446063344540995395360\
      815557919878168193309083573295900545539758915028677094752412489256178770608972743880695597825\
      16229851064188563419476497892884550353389340326220747256139"
    );

        let b = construct_raw_elem_from_strings(
      "16",
      "41",
      "47837607866886756167333839869251273774207619337757918597995294777816250058331116325341018110\
      672047217112377476473502060121352842575308793237621563947157630098485131517401073775191194319\
      531549483898334742144138601661120476425524333273122132151927833887323969998955713328783526854\
      198871332313399489386997681827578317938792170918711794684859311697439726596656501594138449739\
      494228617068329664776714484742276158090583495714649193839084110987149118615158361352488488402\
      038894799695420483272708933239751363849397287571692736881031223140446926522431859701738994562\
      9057462766047140854869124473221137588347335081555186814061"
    );

        let ground_truth = construct_raw_elem_from_strings(
      "64",
      "9",
      "11959401966721689041833459967312818443551904834439479649498823694454062514582779081335254527\
      668011804278094369118375515030338210643827198309405390986789407524621282879350268443797798579\
      882887370974583685536034650415280119106381083318280533037981958471830992499738928332195881713\
      549717833078349872346749420456894579484698042729677948671214827924359931649164125398534612434\
      873557154267082416194178621185569039522645873928662298459771027746787279653789590338122122100\
      50972369992385512081817723330993784096234932189292318422025780578511173163060796492543474864\
      07264365691511785213717281118305284397086833770388796703509"
    );

        assert_eq!(ClassyGroup::op_(&ClassyGroup::rep(), &a, &b), ground_truth);
    }

    #[test]
    fn test_op_alternating() {
        let g_anchor = ClassyGroup::unknown_order_elemnew();
        let mut g = ClassyGroup::idnew();
        let mut g_star = ClassyGroup::idnew();

        // g
        g = ClassyGroup::opnew(&g_anchor, &g);

        // g^2, g^* = g^2
        g = ClassyGroup::opnew(&g_anchor, &g);
        g_star = ClassyGroup::opnew(&g, &g_star);

        // g^3
        g = ClassyGroup::opnew(&g_anchor, &g);

        // g^4, g^* = g^2 * g^4 = g^6
        g = ClassyGroup::opnew(&g_anchor, &g);
        g_star = ClassyGroup::opnew(&g, &g_star);

        let ground_truth = construct_raw_elem_from_strings(
      "64",
      "9",
      "11959401966721689041833459967312818443551904834439479649498823694454062514582779081335254527\
      668011804278094369118375515030338210643827198309405390986789407524621282879350268443797798579\
      882887370974583685536034650415280119106381083318280533037981958471830992499738928332195881713\
      549717833078349872346749420456894579484698042729677948671214827924359931649164125398534612434\
      873557154267082416194178621185569039522645873928662298459771027746787279653789590338122122100\
      509723699923855120818177233309937840962349321892923184220257805785111731630607964925434748640\
      7264365691511785213717281118305284397086833770388796703509"
    );

        assert_eq!(ground_truth, g_star);
    }

    #[test]
    fn test_op_complex() {
        // 1. Take g^100, g^200, ..., g^1000.
        // 2. Compute g^* = g^100 * ... * g^1000.
        // 3. For each of g^100, g^200, ..., g^1000 compute the inverse of that element and assert that
        //    g^* * current_inverse = product of g^100, g^200, ..., g^1000 without the inversed-out
        //    element.
        let g_anchor = ClassyGroup::unknown_order_elemnew();
        let mut g = ClassyGroup::idnew();

        let mut gs = vec![];
        let mut gs_invs = vec![];

        let mut g_star = ClassyGroup::idnew();
        for i in 1..=1000 {
            g = ClassyGroup::opnew(&g_anchor, &g);
            
            if i % 100 == 0 {
                gs.push(g.clone());
                gs_invs.push(ClassyGroup::invnew(&g));
                g_star = ClassyGroup::opnew(&g, &g_star);
                
            }
        }

        let elems_n_invs = gs.iter().zip(gs_invs.iter());
        for (g_elem, g_inv) in elems_n_invs {
            let mut curr_prod = ClassyGroup::idnew();
            for elem in &gs {
                if elem != g_elem {
                    curr_prod = ClassyGroup::opnew(&curr_prod, &elem);
                    
                }
            }
            assert_eq!(ClassyGroup::idnew(), ClassyGroup::opnew(&g_inv, &g_elem));
            assert_eq!(curr_prod, ClassyGroup::opnew(&g_inv, &g_star));
        }
    }

    #[test]
    fn test_id_basic() {
        let g = ClassyGroup::unknown_order_elemnew();
        let id = ClassyGroup::idnew();
        assert_eq!(g, ClassyGroup::opnew(&g, &id));
        assert_eq!(g, ClassyGroup::opnew(&id, &g));
        assert_eq!(id, ClassyGroup::opnew(&id, &id));
    }

    #[test]
    fn test_id_repeated() {
        let mut id = ClassyGroup::idnew();
        let g_anchor = ClassyGroup::unknown_order_elemnew();
        let mut g = ClassyGroup::unknown_order_elemnew();
        for _ in 0..1000 {
            id = ClassyGroup::opnew(&id, &id);
            assert_eq!(id, ClassyGroup::idnew());
            g = ClassyGroup::opnew(&g, &ClassyGroup::idnew());
            assert_eq!(g, g_anchor);
        }
    }

    #[test]
    fn test_inv() {
        let id = ClassyGroup::idnew();
        let g_anchor = ClassyGroup::unknown_order_elemnew();
        let mut g = ClassyGroup::unknown_order_elemnew();

        for _ in 0..1000 {
            g = ClassyGroup::opnew(&g, &g_anchor);
            let g_inv = ClassyGroup::invnew(&g);
            assert_eq!(id, ClassyGroup::opnew(&g_inv, &g));
            assert_eq!(id, ClassyGroup::opnew(&g, &g_inv));
            assert_eq!(g, ClassyGroup::invnew(&g_inv));
        }
    }

    #[test]
    fn test_exp_basic() {
        let g_anchor = ClassyGroup::unknown_order_elemnew();
        let mut g = ClassyGroup::idnew();

        for i in 1..=1000 {
            g = ClassyGroup::opnew(&g, &g_anchor);
            assert_eq!(&g, &ClassyGroup::pow(&g_anchor, &Integer::from(i)));
        }
    }

    #[test]
    fn test_square_basic() {
        let g = ClassyGroup::unknown_order_elemnew();
        let mut g4 = ClassyGroup::idnew();

        // g^4
        for _ in 0..4 {
            g4 = ClassyGroup::opnew(&g, &g4);
        }

        // g^2
        let mut g2 = g.clone();
        // g^4
        ClassyGroup::square(&mut g2);
        ClassyGroup::square(&mut g2);

        assert_eq!(&g2, &g4);
    }

    #[test]
    fn test_square_repeated() {
        let mut g = ClassyGroup::unknown_order_elemnew();
        let g_ = g.clone();

        for i in 0..12 {
            ClassyGroup::square(&mut g);
            let mut base = ClassyGroup::idnew();

            for _ in 0..(2i32.pow(i + 1)) {
                base = ClassyGroup::opnew(&g_, &base);
            }

            assert_eq!(g, base);
        }
    }

    #[test]
    fn test_hash_to_prime_mpz() {
        let b_1 = b"boom i got ur boyfriend";
        let b_2 = b"boom i got ur boyfriene";
        assert_ne!(b_1, b_2);
        let m_1 = ClassyGroup::pick_prime_mpz(b_1); //Mpz
        let m_2 = ClassyGroup::pick_prime_mpz(b_2);
        assert_ne!(m_1, m_2);
        assert!(m_1.is_prime(50)); // resonable value is between 15 and 50.
        assert!(m_2.is_prime(50));
    }


}
