use boostvoronoi::builder as VB;
use boostvoronoi::file_reader as FR;
use boostvoronoi::BvError;
use std::io::{BufReader, Cursor};

mod common;
use common::almost_equal;

type I = i32;
type F = f64;

#[test]
fn sample_primary_066() -> Result<(), BvError> {
    let output = {
        let input = r#"0
193
-50004 -49132 -49976 -49143
-50000 -49669 -49975 -49629
-49998 -49048 -49946 -49035
-49992 -49789 -49931 -49697
-49987 -49783 -49900 -49859
-49986 -49922 -49973 -49927
-49986 -49478 -49972 -49515
-49984 -49089 -49938 -49107
-49983 -49675 -49963 -49734
-49975 -49538 -49940 -49498
-49971 -49867 -49918 -49886
-49964 -49980 -49952 -50034
-49952 -49566 -49904 -49592
-49952 -49236 -49859 -49215
-49945 -49509 -49921 -49460
-49941 -49836 -49865 -49921
-49940 -49321 -49932 -49403
-49933 -49797 -49837 -49806
-49929 -49381 -49858 -49388
-49928 -49608 -49872 -49696
-49927 -49805 -49902 -49825
-49909 -49026 -49858 -48938
-49898 -49229 -49851 -49251
-49895 -49633 -49808 -49721
-49886 -49687 -49837 -49759
-49878 -49348 -49846 -49395
-49875 -49084 -49837 -49109
-49861 -49927 -49842 -49904
-49857 -49151 -49830 -49181
-49856 -49372 -49846 -49393
-49855 -49515 -49784 -49607
-49851 -49037 -49844 -49073
-49845 -49220 -49790 -49301
-49838 -49482 -49826 -49430
-49836 -49490 -49770 -49397
-49829 -49370 -49738 -49326
-49805 -49616 -49781 -49647
-49802 -49947 -49754 -49975
-49802 -49824 -49782 -49751
-49800 -49734 -49712 -49767
-49795 -49462 -49730 -49430
-49788 -49139 -49728 -49156
-49786 -49538 -49747 -49514
-49783 -49261 -49742 -49187
-49775 -49133 -49752 -49051
-49772 -49826 -49770 -49755
-49770 -49737 -49715 -49701
-49768 -49969 -49737 -49988
-49768 -49289 -49708 -49270
-49756 -49890 -49745 -49906
-49732 -50045 -49659 -49957
-49726 -49961 -49689 -49933
-49725 -49584 -49701 -49520
-49717 -49454 -49665 -49408
-49713 -49620 -49696 -49597
-49705 -49300 -49688 -49280
-49702 -48946 -49619 -49011
-49701 -49797 -49672 -49733
-49699 -49730 -49640 -49673
-49696 -49538 -49668 -49562
-49695 -49454 -49644 -49509
-49683 -49845 -49613 -49847
-49676 -49455 -49614 -49455
-49676 -49165 -49665 -49131
-49669 -49436 -49618 -49397
-49667 -49273 -49611 -49341
-49664 -49941 -49644 -49853
-49655 -49426 -49593 -49405
-49650 -49039 -49592 -48996
-49645 -49762 -49558 -49819
-49642 -49918 -49629 -49925
-49638 -49409 -49582 -49328
-49636 -49743 -49559 -49805
-49629 -49529 -49549 -49487
-49627 -49096 -49577 -49054
-49626 -49940 -49562 -50025
-49612 -49466 -49574 -49398
-49606 -49583 -49532 -49633
-49606 -49288 -49556 -49356
-49600 -49856 -49556 -49907
-49594 -49166 -49546 -49148
-49593 -49160 -49557 -49119
-49586 -49533 -49534 -49534
-49576 -49655 -49527 -49733
-49571 -49468 -49564 -49410
-49555 -49534 -49505 -49544
-49555 -49314 -49503 -49289
-49547 -49413 -49535 -49473
-49543 -49984 -49462 -49886
-49537 -49835 -49489 -49866
-49536 -49256 -49515 -49237
-49530 -49940 -49503 -49875
-49510 -49342 -49485 -49366
-49505 -49287 -49455 -49308
-49499 -49909 -49475 -49880
-49499 -49648 -49486 -49612
-49489 -49785 -49482 -49851
-49488 -49690 -49469 -49624
-49484 -49727 -49441 -49650
-49477 -49146 -49461 -49149
-49474 -49433 -49401 -49411
-49471 -49873 -49451 -49780
-49471 -49725 -49415 -49775
-49460 -49998 -49361 -49980
-49458 -49621 -49448 -49561
-49448 -49432 -49420 -49438
-49443 -49942 -49353 -49975
-49441 -49662 -49360 -49721
-49439 -49313 -49375 -49361
-49438 -49826 -49378 -49771
-49437 -49884 -49427 -49849
-49432 -49713 -49416 -49748
-49423 -49035 -49363 -48938
-49411 -49201 -49312 -49232
-49410 -49095 -49399 -49034
-49399 -49547 -49342 -49497
-49399 -49230 -49390 -49259
-49394 -49445 -49361 -49429
-49393 -49615 -49360 -49660
-49392 -49185 -49371 -49185
-49392 -49020 -49390 -49005
-49369 -49816 -49324 -49864
-49368 -49828 -49342 -49845
-49355 -50082 -49274 -49996
-49354 -49414 -49270 -49512
-49350 -49378 -49324 -49448
-49349 -49027 -49317 -49012
-49345 -49337 -49315 -49263
-49344 -49983 -49278 -49977
-49341 -49596 -49321 -49518
-49338 -49054 -49329 -49121
-49337 -49788 -49267 -49727
-49333 -49258 -49320 -49275
-49329 -49596 -49276 -49520
-49325 -49228 -49232 -49257
-49316 -49037 -49266 -49136
-49313 -49612 -49267 -49604
-49310 -49276 -49305 -49362
-49306 -49704 -49294 -49652
-49302 -49872 -49216 -49849
-49286 -49089 -49225 -49139
-49284 -49119 -49278 -49117
-49276 -49306 -49209 -49406
-49271 -49927 -49196 -49833
-49264 -49696 -49176 -49674
-49253 -49909 -49245 -49992
-49252 -49804 -49161 -49822
-49251 -49757 -49177 -49747
-49249 -49603 -49150 -49595
-49239 -49989 -49200 -49929
-49239 -49202 -49205 -49178
-49233 -49713 -49169 -49751
-49225 -49327 -49210 -49359
-49223 -49037 -49158 -48993
-49219 -49243 -49181 -49336
-49212 -49235 -49199 -49267
-49211 -49912 -49138 -49959
-49210 -49462 -49123 -49497
-49206 -49814 -49151 -49905
-49201 -49527 -49188 -49481
-49190 -49120 -49173 -49206
-49178 -49656 -49174 -49598
-49172 -49616 -49110 -49600
-49164 -49673 -49157 -49768
-49161 -49414 -49096 -49406
-49159 -49013 -49077 -49078
-49154 -49000 -49086 -49047
-49151 -49787 -49151 -49693
-49143 -49651 -49123 -49683
-49141 -49982 -49100 -49949
-49138 -49607 -49117 -49583
-49128 -49565 -49115 -49499
-49123 -49412 -49040 -49426
-49121 -49732 -49103 -49690
-49117 -49724 -49102 -49688
-49114 -49206 -49075 -49124
-49110 -49937 -49030 -49889
-49109 -49215 -49071 -49305
-49092 -49527 -49056 -49604
-49091 -49133 -49057 -49070
-49089 -49036 -49022 -49076
-49079 -49254 -49019 -49331
-49077 -49768 -49042 -49804
-49060 -49611 -49029 -49677
-49060 -49083 -49052 -49170
-49054 -49751 -49041 -49789
-49047 -49846 -48991 -49882
-49045 -49567 -48976 -49528
-49043 -49787 -49000 -49864
-49038 -49415 -49028 -49463
-49033 -49721 -48973 -49756
-49009 -49823 -48919 -49827
-49008 -49022 -48959 -49051
"#;
        let mut vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        let output = vb.build()?;
        #[cfg(feature = "geo")]
        common::diagram_sanity_check::<I, F>(&output, &points, &segments, 0.0001)?;
        output
    };

    // A full test would take 7300 loc
    assert_eq!(output.cells().len(), 579);
    assert_eq!(output.vertices().len(), 1142);
    assert_eq!(output.edges().len(), 3440);

    let v = output.vertices()[626].get();
    assert!(almost_equal(v.x(), -49456.7929234, v.y(), -49236.9315545));
    assert_eq!(v.get_incident_edge().unwrap().0, 1935);

    let v = output.vertices()[1066].get();
    assert!(almost_equal(v.x(), -49041.0986560, v.y(), -49432.5134604));
    assert_eq!(v.get_incident_edge().unwrap().0, 3255);

    let v = output.vertices()[1079].get();
    assert!(almost_equal(v.x(), -49046.1927997, v.y(), -49824.7984762));
    assert_eq!(v.get_incident_edge().unwrap().0, 3291);

    let v = output.vertices()[1108].get();
    assert!(almost_equal(v.x(), -49000.0771171, v.y(), -49784.8205615));
    assert_eq!(v.get_incident_edge().unwrap().0, 3369);

    let v = output.vertices()[1138].get();
    assert!(almost_equal(v.x(), -49610.5000000, v.y(), -45636.7500000));
    assert_eq!(v.get_incident_edge().unwrap().0, 3433);

    Ok(())
}
