extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

#[test]
fn sutra_3_3_10() {
    assert_has_krdanta(&[], &d("Bu\\ja~", Rudhadi), Krt::tumun, &["Boktum"]);
}

#[test]
fn sutra_3_3_13() {
    assert_has_lrt_p(&[], &d("qukf\\Y", Tanadi), &["karizyati"]);
    assert_has_lrt_p(&[], &d("hf\\Y", Bhvadi), &["harizyati"]);
}

#[test]
fn sutra_3_3_15() {
    assert_has_lut_p(&[], &d("qukf\\Y", Tanadi), &["kartA"]);
    assert_has_lut_p(&[], &d("Bu\\ja~", Rudhadi), &["BoktA"]);
}

#[test]
fn sutra_3_3_16() {
    assert_has_krdanta(&[], &d("pa\\da~\\", Divadi), Krt::GaY, &["pAda"]);
    assert_has_krdanta(&[], &d("ru\\jo~", Tudadi), Krt::GaY, &["roga"]);
    assert_has_krdanta(&[], &d("vi\\Sa~", Tudadi), Krt::GaY, &["veSa"]);
    assert_has_krdanta(&[], &d("spf\\Sa~", Tudadi), Krt::GaY, &["sparSa"]);
}

#[test]
fn sutra_3_3_17() {
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), Krt::GaY, &["sAra"]);
}

#[test]
fn sutra_3_3_18() {
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), Krt::GaY, &["pAka"]);
    assert_has_krdanta(&[], &d("tya\\ja~", Bhvadi), Krt::GaY, &["tyAga"]);
    assert_has_krdanta(&[], &d("ra\\nja~^", Divadi), Krt::GaY, &["rAga", "raNga"]);
}

#[test]
fn sutra_3_3_89() {
    assert_has_krdanta(&[], &d("wuvepf~\\", Bhvadi), Krt::aTuc, &["vepaTu"]);
    assert_has_krdanta(&[], &d("wuo~Svi", Bhvadi), Krt::aTuc, &["SvayaTu"]);
    assert_has_krdanta(&[], &d("wukzu", Adadi), Krt::aTuc, &["kzavaTu"]);
}

#[test]
fn sutra_3_3_90() {
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), Krt::naN, &["yajYa"]);
    assert_has_krdanta(&[], &d("quyAcf~^", Bhvadi), Krt::naN, &["yAcYA"]);
    assert_has_krdanta(&[], &d("yatI~\\", Bhvadi), Krt::naN, &["yatna"]);
    assert_has_krdanta(&[], &d("viCa~", Tudadi), Krt::naN, &["viSna"]);
    assert_has_krdanta(&[], &d("pra\\Ca~", Tudadi), Krt::naN, &["praSna"]);
    assert_has_krdanta(&[], &d("rakza~", Bhvadi), Krt::naN, &["rakzRa"]);
}

#[test]
fn sutra_3_3_91() {
    assert_has_krdanta(&[], &d("Yizva\\pa~", Adadi), Krt::nan, &["svapna"]);
}

#[test]
fn sutra_3_3_94() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::ktin, &["kfti"]);
    assert_has_krdanta(&[], &d("ci\\Y", Svadi), Krt::ktin, &["citi"]);
    assert_has_krdanta(&[], &d("ma\\na~\\", Divadi), Krt::ktin, &["mati"]);
    // TODO: others
}

#[test]
fn sutra_3_3_115() {
    assert_has_krdanta(&[], &d("hase~", Bhvadi), Krt::lyuw, &["hasana"]);
    assert_has_krdanta(&[], &d("SuBa~\\", Bhvadi), Krt::lyuw, &["SoBana"]);
    assert_has_krdanta(&[], &d("jalpa~", Bhvadi), Krt::lyuw, &["jalpana"]);
    assert_has_krdanta(&[], &d("SIN", Adadi), Krt::lyuw, &["Sayana"]);
    assert_has_krdanta(&[], &d("Asa~\\", Adadi), Krt::lyuw, &["Asana"]);
}
