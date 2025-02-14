use crate::dhatu_gana as gana;
use crate::operators as op;
/// Applies samprasarana changes as needed.
///
/// Order of operations:
/// - Must follow atidesha so that suffixes have the kit/Nit annotations necessary to cause
///   samprasanara.
use crate::prakriya::{Code, Prakriya};
use crate::tag::Tag as T;
use crate::term::Term;

fn is_vaci_svapi(t: &Term) -> bool {
    t.is_dhatu()
        && (t.has_u_in(&["va\\ca~", "Yizva\\pa~"])
            || t.has_u_in(gana::YAJ_ADI)
            || t.has_u("va\\ci~"))
}

fn is_grahi_jya(t: &Term) -> bool {
    t.is_dhatu()
        && t.has_u_in(&[
            "graha~^",
            "jyA\\",
            // vayi~ replaces ve\\Y in 2.4.41
            "vayi~",
            "vya\\Da~",
            "vaSa~",
            "vyaca~",
            "o~vrascU~",
            "pra\\Ca~",
            "Bra\\sja~^",
            // not sure how to handle "vay" root
            "vaya~\\",
        ])
}

/// Runs a hacky version of samprasarana that runs 6.1.108 (samprasAraNAcca) immediately.
///
/// TODO: properly annotate 6.1.108 and related rules here.
fn do_samprasarana(rule: Code, p: &mut Prakriya, i: usize) -> Option<()> {
    let before = &[
        "vac", "svap", "yaj", "vap", "vah", "vas", "ve", "vye", "hve", "vad", "Svi",
        // grahi-jyA
        "grah", "jyA", "vay", "vyaD", "vaS", "vyac", "vrasc", "praC", "Brasj",
        // other rules
        "syam",
    ];
    let after = &[
        "uc", "sup", "ij", "up", "uh", "us", "u", "vI", "hU", "ud", "SU", // grahi-jyA
        "gfh", "ji", "uy", "viD", "uS", "vic", "vfSc", "pfC", "Bfsj", // other rules
        "sim",
    ];
    let text = &p.get(i)?.text;
    if let Some(j) = before.iter().position(|x| x == text) {
        p.op_term(rule, i, |t| {
            t.set_text(after[j]);
            t.add_tag(T::FlagSamprasarana);
        });
    }
    Some(())
}

pub fn run_for_dhatu(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;

    let dhatu = p.get(i)?;
    let n = p.view(i_n)?;
    let n_is_yan = n.has_u("yaN");
    let n_is_lit = n.has_lakshana("li~w");
    let n_will_be_abhyasta = n_is_lit || n.has_u_in(&["san", "yaN", "Slu", "caN"]);

    let set_text = |rule, p: &mut Prakriya, text| {
        p.op_term(rule, i, |t| {
            t.set_text(text);
            t.add_tag(T::FlagSamprasarana);
        });
    };

    let optional_set_text = |rule, p: &mut Prakriya, text| {
        p.op_optional(
            rule,
            op::t(i, |t| {
                t.set_text(text);
                t.add_tag(T::FlagSamprasarana);
            }),
        );
    };

    let is_ve = dhatu.has_u("ve\\Y");
    if dhatu.has_u("Yizva\\pa~") && n.has_u("Ric") && p.has(i_n + 1, |t| t.has_u("caN")) {
        // asUzupat
        do_samprasarana("6.1.18", p, i);
    } else if dhatu.has_u_in(&["Yizva\\pa~", "syamu~", "vye\\Y"]) && n_is_yan {
        // sozupyate, sesimyate, vevIyate
        do_samprasarana("6.1.19", p, i);
    } else if dhatu.has_u("vaSa~") && n_is_yan {
        // vAvaSyate (exception to grahi-jyA-...)
        p.step("6.1.20");
    } else if dhatu.has_u("cAyf~^") && n_is_yan {
        // cekIyate
        set_text("6.1.21", p, "kI");
    } else if dhatu.has_u("sPAyI~\\") && n.has_tag(T::Nistha) {
        // sPIta
        set_text("6.1.22", p, "sPI");
    } else if dhatu.has_text("styE")
        && n.has_tag(T::Nistha)
        && i > 0
        && p.has(i - 1, |t| t.has_u("pra"))
    {
        // prastIta
        set_text("6.1.23", p, "sti");
    } else if dhatu.has_u("o~pyAyI~\\") && n.has_tag(T::Nistha) {
        let code = "6.1.28";
        if i == 0 {
            set_text(code, p, "pI");
        } else {
            optional_set_text(code, p, "pI");
        }
    } else if dhatu.has_u("o~pyAyI~\\") && (n_is_yan || n_is_lit) {
        set_text("6.1.29", p, "pI");
    } else if dhatu.has_text("Svi") && (n_is_yan || n_is_lit) {
        optional_set_text("6.1.30", p, "Su");
    } else if dhatu.has_text("hve") && n_will_be_abhyasta {
        set_text("6.1.33", p, "hu");
    } else if is_ve && n.has_lakshana("li~w") {
        p.step("6.1.40");
    } else if is_ve && n.has_u("lyap") {
        p.step("6.1.41");
    } else if dhatu.has_u("jyA\\") && n.has_u("lyap") {
        p.step("6.1.42");
    } else if dhatu.has_u("vye\\Y") && n.has_u("lyap") {
        if i > 0 && p.has(i - 1, |t| t.has_u("pari")) {
            optional_set_text("6.1.44", p, "vi");
        } else {
            p.step("6.1.43");
        }
    } else {
        // General rules
        if is_vaci_svapi(dhatu) && n.has_tag(T::kit) {
            do_samprasarana("6.1.15", p, i);
        } else if is_grahi_jya(dhatu) && n.is_knit() {
            if dhatu.has_u("pra\\Ca~") && n.has_u("naN") {
                // Per ashtadhyayi.com, skip samprasarana for praC + naN.
            } else {
                do_samprasarana("6.1.16", p, i);
                if p.has(i, |t| t.has_text("uy") && t.has_u("vayi~")) {
                    optional_set_text("6.1.39", p, "uv");
                }
            }
        }
    }

    Some(())
}

pub fn run_for_abhyasa(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Abhyasa)?;
    let dhatu = p.get_if(i + 1, |t| t.is_dhatu())?;
    let last = p.terms().last()?;

    if last.has_lakshana("li~w") {
        // yadā ca dhātorna bhavati tadā "liṭyabhyāsasya ubhayeṣām"
        // ityabhyāsasya api na bhavati -- kāśikā.
        if is_vaci_svapi(dhatu) && !dhatu.has_text("Svi") {
            if dhatu.has_u("ve\\Y") {
                p.step("6.1.40");
            } else {
                do_samprasarana("6.1.17", p, i);
            }
        } else if is_grahi_jya(dhatu) {
            do_samprasarana("6.1.17", p, i);
        }
    }

    Some(())
}
