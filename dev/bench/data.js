window.BENCHMARK_DATA = {
  "lastUpdate": 1764196414123,
  "repoUrl": "https://github.com/timvschool/frunk",
  "entries": {
    "OrdoFP Benchmarks": [
      {
        "commit": {
          "author": {
            "email": "tvail@wgu.edu",
            "name": "TimV",
            "username": "timvschool"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "11c0b2b4a9f272e83e3b09f022e67d168b092f4f",
          "message": "refactor(ordofp): rename Frunk to OrdoFP and adopt SOTA updates across crates (#1)\n\nThis renames the codebase branding from Frunk to OrdoFP and upgrades to 2025 SOTA standards.\n\n- Rename crates to ordofp_core, ordofp_derives, ordofp_proc_macros, ordofp_laws, and related helpers.\n- Introduce GAT-based Functor/Applicative/Monad and expose via prelude.\n- Add comprehensive error guide and progressive examples.\n- Modernize CI/CD and add fuzzing infrastructure; update docs.\n- Update changelog and README to reflect new branding.\n\nNote: This is a breaking change; user code must migrate from frunk to ordofp crate names and imports, and adapt to the new crate structure.\n\nCo-authored-by: Tim <trvail@liberty.edu>",
          "timestamp": "2025-11-26T22:32:10Z",
          "tree_id": "ce3a8149712899ee4a73968231550e1b94c3e16e",
          "url": "https://github.com/timvschool/frunk/commit/11c0b2b4a9f272e83e3b09f022e67d168b092f4f"
        },
        "date": 1764196412969,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0.29,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 0.86,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 0.86,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 0.86,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 1.73,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 0.86,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 0.86,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 0.86,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 0.86,
            "range": "± 0.09",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 0.86,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 51.44,
            "range": "± 0.97",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 61.21,
            "range": "± 1.88",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 1926.96,
            "range": "± 6.08",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 1906.89,
            "range": "± 14.68",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 0.86,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0.29,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 0.86,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 10.08,
            "range": "± 0.07",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 3.38,
            "range": "± 0.68",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0.29,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0.29,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0.29,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0.29,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0.29,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0.29,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1.15,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 27.19,
            "range": "± 0.30",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0.29,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 41.7,
            "range": "± 0.77",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 501.96,
            "range": "± 8.00",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 419.47,
            "range": "± 6.48",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 212.54,
            "range": "± 2.55",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 19,
            "range": "± 0.11",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 90.83,
            "range": "± 1.29",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 13.52,
            "range": "± 0.04",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 26.08,
            "range": "± 0.43",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 1.26,
            "range": "± 0.13",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 0.58,
            "range": "± 0.01",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}