<!-- lightcraft ci report, schema 1 -->
**`cargo xtask ci`: fail** at b11d21de44 (clean)

| | |
|---|---|
| Machine | linux x86_64 (Ubuntu 24.04.5 LTS, kernel 6.17.0-1033-oem), 8 CPUs, 31 GB RAM (21.7 GB available) |
| Rust | rustc 1.95.0 (59807616e 2026-04-14) |
| GPU for the pipeline | Intel(R) Graphics (LNL) (Vulkan) |
| Adapters | vulkan: Intel(R) Graphics (LNL) (IntegratedGpu; Intel open-source Mesa driver Mesa 25.2.8-0ubuntu0.24.04.2), llvmpipe (LLVM 20.1.2, 256 bits) (Cpu; llvmpipe Mesa 25.2.8-0ubuntu0.24.04.2 (LLVM 20.1.2)) |
| Data | corpus: 0 of 451 pinned raws, 0 images, 0 PngSuite files; no CRAFT_FONTS_DIR |

| Step | Result | Time |
|---|---|---|
| fmt | ok | 3.7 s |
| clippy | ok | 2m 48s |
| heif | ok | 5m 03s |
| test | fail | 23m 38s |
| parity | not run |  |
| layers | not run |  |
| assets | not run |  |
| wasm | not run |  |

<details><summary>175 tests skipped (corpus 137, fonts 28, model 8, gpu 1, system 1)</summary>

| Need | Tests | Reason | For example |
|---|---|---|---|
| corpus | 7 | corpus/raw absent | `raw/it::corpus::corpus_dngs_carry_profile_looks` |
| corpus | 3 | corpus/raw/arw-sony-a7rm2-12bit-uncompressed.arw absent | `lightcraft_engine::camera_preview::tests::corpus_arw_with_a_lens_corrected_preview_gets_a_camera_look` |
| corpus | 3 | cr2-canon-5d3.cr2 absent | `lightcraft_engine::camera_preview::tests::corpus_colour_precedence` |
| corpus | 3 | cr2-canon-7d.cr2 absent | `lightcraft_engine::camera_preview::tests::corpus_colour_precedence` |
| corpus | 2 | arw-sony-a500.arw absent | `raw/it::corpus::corpus_sony_black_levels_sit_at_the_data_floor` |
| corpus | 2 | arw-sony-a700.arw absent | `raw/it::corpus::corpus_sony_black_levels_sit_at_the_data_floor` |
| corpus | 2 | arw-sony-a7cm2-compressed-m.arw absent | `raw/it::corpus::corpus_sony_a7cm2_codings` |
| corpus | 2 | arw-sony-a7cm2-lossless-l.arw absent | `raw/it::corpus::corpus_sony_a7cm2_codings` |
| corpus | 2 | arw-sony-a7cr-compressed.arw absent | `lightcraft_engine::camera_preview::tests::corpus_a7cr_codings_get_a_camera_look` |
| corpus | 2 | arw-sony-a7cr-lossless-l.arw absent | `lightcraft_engine::camera_preview::tests::corpus_a7cr_codings_get_a_camera_look` |
| corpus | 2 | arw-sony-a7cr-lossless-m.arw absent | `lightcraft_engine::camera_preview::tests::corpus_a7cr_codings_get_a_camera_look` |
| corpus | 2 | arw-sony-a900-packed12.arw absent | `raw/it::corpus::corpus_sony_a900_packed12` |
| corpus | 105 | 92 other reasons | |
| fonts | 2 | ja: built without a craft-fonts face for Jpan | `lightcraft_ui_egui::i18n::tests::catalog_characters_are_paintable` |
| fonts | 2 | zh-hans: built without a craft-fonts face for Hans | `lightcraft_ui_egui::i18n::tests::catalog_characters_are_paintable` |
| fonts | 2 | zh-hant: built without a craft-fonts face for Hant | `lightcraft_ui_egui::i18n::tests::catalog_characters_are_paintable` |
| fonts | 1 | Japanese comparisons: built without BIZ UDMincho from craft-fonts | `lightcraft_engine::export::tests::vertical_watermarks_keep_combining_marks_in_one_cell` |
| fonts | 1 | built without BIZ UDMincho from craft-fonts | `lightcraft_engine::export::tests::biz_ud_vertical_watermarks_place_punctuation_at_the_top_right` |
| fonts | 1 | built without CJK faces from craft-fonts | `lightcraft_engine::text::tests::latin_and_cjk_rendering_keeps_the_existing_preference` |
| fonts | 1 | built without CRAFT_FONTS_DIR, so there is no Japanese watermark face | `lightcraft_engine::export::tests::japanese_watermarks_support_vertical_columns` |
| fonts | 1 | built without CRAFT_FONTS_DIR: glyphs only craft-fonts has are drawn as boxes, not checked | `ui-egui/it::accessibility::every_interactive_node_has_a_name_in_every_view_and_tool` |
| fonts | 1 | built without CRAFT_FONTS_DIR: no craft-fonts to check | `lightcraft_ui_egui::theme::tests::thai_folder_name_has_glyphs_in_every_ui_family` |
| fonts | 1 | built without the Arabic face from craft-fonts | `lightcraft_engine::text::tests::bundled_arabic_stays_out_of_export_fallback` |
| fonts | 1 | built without the Thai face from craft-fonts | `lightcraft_engine::text::tests::bundled_thai_uses_its_glyphs` |
| fonts | 1 | cs: built without a craft-fonts face for Latn | `lightcraft_ui_egui::i18n::tests::catalog_characters_are_paintable` |
| fonts | 13 | 13 other reasons | |
| model | 4 | LC_YUNET_MODEL is not set | `lightcraft_engine::tests::face_models::detect_replaces_only_earlier_detections_and_is_one_undo_step` |
| model | 3 | LC_YUNET_MODEL is not set, so the face-finding part is not checked | `lightcraft_engine::cmd::face_recognize::imp::tests::only_the_background_scan_looks_for_faces_and_only_in_photos_without_any` |
| model | 1 | set LIGHTCRAFT_SAM3_DIR and LIGHTCRAFT_SAM3_REF | `segment/it::reference::matches_reference` |
| gpu | 1 | no GPU device, or one that can't be lost on purpose here | `lightcraft_gpu::ctx::tests::a_really_lost_device_is_not_polled_again` |
| system | 1 | no system ICC profiles | `codecs/it::corpus::system_icc_profiles` |

</details>
