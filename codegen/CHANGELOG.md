# Changelog

* The following workspace dependencies were updated
  * dependencies
    * substrate_api bumped from 0.1.0 to 0.1.1
  * dev-dependencies
    * substrate bumped from 0.1.0 to 0.1.1

## [0.7.0](https://github.com/substrate-labs/substrate2/compare/codegen-v0.6.1...codegen-v0.7.0) (2023-08-16)


### Features

* **impl-dispatch:** remove impl dispatch in favor of trait bounds ([#283](https://github.com/substrate-labs/substrate2/issues/283)) ([d954115](https://github.com/substrate-labs/substrate2/commit/d9541152db52aebde928e41c0d800453e906d62b))
* **netlister:** reduce duplicate code between spectre and SPICE netlisters ([#261](https://github.com/substrate-labs/substrate2/issues/261)) ([5ba3623](https://github.com/substrate-labs/substrate2/commit/5ba36230e653e4dc77819c5c50b527311768cd83))
* **refactor:** rename Has_ and Has_Data ([#282](https://github.com/substrate-labs/substrate2/issues/282)) ([2018153](https://github.com/substrate-labs/substrate2/commit/2018153686dd7ef3df0e10874db3c656ca245026))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * examples bumped from 0.3.1 to 0.4.0
  * dev-dependencies
    * substrate bumped from <=0.6.1 to <=0.7.0
    * sky130pdk bumped from <=0.6.1 to <=0.7.0
    * spectre bumped from <=0.6.1 to <=0.7.0

## [0.6.1](https://github.com/substrate-labs/substrate2/compare/codegen-v0.6.0...codegen-v0.6.1) (2023-08-08)


### Bug Fixes

* **generics:** change `Deserialize&lt;'static&gt;` bounds to `DeserializeOwned` ([#259](https://github.com/substrate-labs/substrate2/issues/259)) ([8015063](https://github.com/substrate-labs/substrate2/commit/80150630b094a04a75cfc5b681255b80caf4f895))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * examples bumped from 0.3.0 to 0.3.1
  * dev-dependencies
    * substrate bumped from <=0.6.0 to <=0.6.1
    * sky130pdk bumped from <=0.6.0 to <=0.6.1
    * spectre bumped from <=0.6.0 to <=0.6.1

## [0.6.0](https://github.com/substrate-labs/substrate2/compare/codegen-v0.5.0...codegen-v0.6.0) (2023-08-08)


### Features

* **codegen:** derive Block macro adds required trait bounds by default ([#249](https://github.com/substrate-labs/substrate2/issues/249)) ([892bef5](https://github.com/substrate-labs/substrate2/commit/892bef585548264e3fcdcc2e6523a2321c6c6897))
* **codegen:** insert appropriate bounds in Io, SchematicType, LayoutType proc macros ([#251](https://github.com/substrate-labs/substrate2/issues/251)) ([33dcc79](https://github.com/substrate-labs/substrate2/commit/33dcc797fdbeb21ad046093e655acf965fd99321))
* **macros:** refactor macro reexports ([#250](https://github.com/substrate-labs/substrate2/issues/250)) ([a332717](https://github.com/substrate-labs/substrate2/commit/a332717e549fdea50306067e1c92dc60293aed4c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * examples bumped from 0.2.0 to 0.3.0
    * type_dispatch bumped from 0.2.0 to 0.3.0
  * dev-dependencies
    * substrate bumped from <=0.5.0 to <=0.6.0
    * sky130pdk bumped from <=0.5.0 to <=0.6.0
    * spectre bumped from <=0.5.0 to <=0.6.0

## [0.5.0](https://github.com/substrate-labs/substrate2/compare/codegen-v0.4.0...codegen-v0.5.0) (2023-08-05)


### Features

* **codegen:** derive macro for implementing FromSaved ([#243](https://github.com/substrate-labs/substrate2/issues/243)) ([48acae0](https://github.com/substrate-labs/substrate2/commit/48acae0fb8915c4f968223268c92077f2deda979))
* **terminals:** add support for terminal paths ([#236](https://github.com/substrate-labs/substrate2/issues/236)) ([3fba7f6](https://github.com/substrate-labs/substrate2/commit/3fba7f6227bbf2efcaf79d849c79175e44d783a4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * examples bumped from 0.1.0 to 0.2.0
  * dev-dependencies
    * substrate bumped from <=0.4.0 to <=0.5.0
    * sky130pdk bumped from <=0.4.0 to <=0.5.0
    * spectre bumped from <=0.4.0 to <=0.5.0

## [0.4.0](https://github.com/substrate-labs/substrate2/compare/codegen-v0.3.0...codegen-v0.4.0) (2023-08-04)


### Features

* **corners:** require specifying corner by default ([#221](https://github.com/substrate-labs/substrate2/issues/221)) ([4c2c3e4](https://github.com/substrate-labs/substrate2/commit/4c2c3e4a3cd8b7e68921baf3af8b87f1da048936))
* **docs:** reorganize docs and add code snippets ([#216](https://github.com/substrate-labs/substrate2/issues/216)) ([d7c457d](https://github.com/substrate-labs/substrate2/commit/d7c457d4e5c1d4846549a0e6df958243042285db))
* **layout:** rename `HasLayout` and `HasLayoutImpl` ([#227](https://github.com/substrate-labs/substrate2/issues/227)) ([2cf1f7d](https://github.com/substrate-labs/substrate2/commit/2cf1f7d435549df26ff15370e7324e9df76e0e4f))
* **parameters:** substrate schematic primitives support parameters ([#233](https://github.com/substrate-labs/substrate2/issues/233)) ([5dabcb2](https://github.com/substrate-labs/substrate2/commit/5dabcb270cab0d259b7301d67f77de6d55261092))
* **pdk:** remove `PdkData` object to clean up interface ([#218](https://github.com/substrate-labs/substrate2/issues/218)) ([1dd166a](https://github.com/substrate-labs/substrate2/commit/1dd166a8f23e7b3c011c01b5c8527b8c5494ddea))
* **schematics:** rename `HasSchematic` and `HasSchematicImpl` ([#226](https://github.com/substrate-labs/substrate2/issues/226)) ([a2b9c78](https://github.com/substrate-labs/substrate2/commit/a2b9c78ea6ff56983e9a02aeafe655e92852c264))
* **schematics:** user-specified schematic hierarchy flattening ([#222](https://github.com/substrate-labs/substrate2/issues/222)) ([251f377](https://github.com/substrate-labs/substrate2/commit/251f37778526d2f1c08a2b3c66f72ffe273021fa))
* **spectre:** vsource uses primitives instead of being blackboxed ([5dabcb2](https://github.com/substrate-labs/substrate2/commit/5dabcb270cab0d259b7301d67f77de6d55261092))
* **type-dispatch:** add helper crate for dispatching types ([#225](https://github.com/substrate-labs/substrate2/issues/225)) ([13ee1aa](https://github.com/substrate-labs/substrate2/commit/13ee1aa1b287ed0c147549003c0af815b849577b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * examples bumped from <=0.0.0 to 0.1.0
  * dev-dependencies
    * substrate bumped from <=0.3.0 to <=0.4.0
    * sky130pdk bumped from <=0.3.0 to <=0.4.0

## [0.3.0](https://github.com/substrate-labs/substrate2/compare/codegen-v0.2.0...codegen-v0.3.0) (2023-07-23)


### Features

* **codegen:** implement derive proc macro for layout hard macros ([#200](https://github.com/substrate-labs/substrate2/issues/200)) ([5138224](https://github.com/substrate-labs/substrate2/commit/5138224013f537e678dfb20204e964852ed40ccb))
* **gds-import:** implement GDS to RawCell importer ([#196](https://github.com/substrate-labs/substrate2/issues/196)) ([fc37eeb](https://github.com/substrate-labs/substrate2/commit/fc37eeb6bac10779491b98bcadcc0eeaeb7d8ec5))
* **proc-macros:** codegen for schematic hard macros ([#191](https://github.com/substrate-labs/substrate2/issues/191)) ([50240b1](https://github.com/substrate-labs/substrate2/commit/50240b167876873c4133315d35298b44e8eeac51))
* **proc-macros:** derive macros for geometry traits ([#164](https://github.com/substrate-labs/substrate2/issues/164)) ([a86074a](https://github.com/substrate-labs/substrate2/commit/a86074a69b714b1be551ae00c775beb04c13f776))
* **proc-macros:** support enums, tuple structs, etc. ([#165](https://github.com/substrate-labs/substrate2/issues/165)) ([bda83f7](https://github.com/substrate-labs/substrate2/commit/bda83f7c3049178024b114eb4e1bf65c6a128998))
* **proc-macros:** support generics in derive schematic/layout data ([#169](https://github.com/substrate-labs/substrate2/issues/169)) ([5bc11d8](https://github.com/substrate-labs/substrate2/commit/5bc11d8eee266c21247694299285b6147631166e))
* **simulation:** proc macros for implementing Supports on tuples ([#163](https://github.com/substrate-labs/substrate2/issues/163)) ([bf77832](https://github.com/substrate-labs/substrate2/commit/bf778329d6e9fd317bea789d093c4c7d8790f5ac))
* **tiling:** array and grid tiling API ([#201](https://github.com/substrate-labs/substrate2/issues/201)) ([b3b7c2b](https://github.com/substrate-labs/substrate2/commit/b3b7c2bfb7ba72198872d0f08ded3e0bc757479d))


### Bug Fixes

* **ci:** add workaround for dev deps ([180c924](https://github.com/substrate-labs/substrate2/commit/180c92434b38a5da8d5d1f0494faae6a0b227c26))
* **ci:** test another workaround for dev deps ([c15bc6d](https://github.com/substrate-labs/substrate2/commit/c15bc6d30afc02512237223db5f31cd9cb089ede))
* **tests:** increase cache server wait time ([#167](https://github.com/substrate-labs/substrate2/issues/167)) ([b0db3aa](https://github.com/substrate-labs/substrate2/commit/b0db3aa6285367de1650e972c9cf7e2185a68250))


### Dependencies

* The following workspace dependencies were updated
  * dev-dependencies
    * substrate bumped from <=0.2.0 to <=0.3.0
    * sky130pdk bumped from <=0.2.0 to <=0.3.0

## [0.2.0](https://github.com/substrate-labs/substrate2/compare/codegen-v0.1.1...codegen-v0.2.0) (2023-07-07)


### Features

* **reorg:** move substrate-api into substrate ([#155](https://github.com/substrate-labs/substrate2/issues/155)) ([e902a1b](https://github.com/substrate-labs/substrate2/commit/e902a1b603cca6c719770c5cd742e081bfd33e51))


### Dependencies

* The following workspace dependencies were updated
  * dev-dependencies
    * substrate bumped from <=0.1.1 to 0.2.0

## 0.1.0 (2023-07-07)


### Features

* **custom-layout-io:** add way to derive custom layout IOs ([#117](https://github.com/substrate-labs/substrate2/issues/117)) ([61a8625](https://github.com/substrate-labs/substrate2/commit/61a86251978fde6e8d1095d33f197d5702d085cc))
* **docs:** add docs for layout IO ([#131](https://github.com/substrate-labs/substrate2/issues/131)) ([551d65e](https://github.com/substrate-labs/substrate2/commit/551d65e440ae3c7a9ccbe5d35a7ed5cd93d0d6b3))
* **gds-export:** add GDS crate and utilities for accessing GDS layers ([#87](https://github.com/substrate-labs/substrate2/issues/87)) ([5cf11cd](https://github.com/substrate-labs/substrate2/commit/5cf11cd0ff80d637ca7210a603625a3b950cdaa4))
* **gds-export:** implement GDS export of Substrate cells ([#97](https://github.com/substrate-labs/substrate2/issues/97)) ([ae5ca3d](https://github.com/substrate-labs/substrate2/commit/ae5ca3d0356848eb8e080a7714667193bb9d28fb))
* **layer-api:** add layer IDs to shapes ([#85](https://github.com/substrate-labs/substrate2/issues/85)) ([df7064d](https://github.com/substrate-labs/substrate2/commit/df7064d0268d1ef7d2ec8bfb5b66434a9b19e819))
* **layer-api:** initial layer API and codegen ([#84](https://github.com/substrate-labs/substrate2/issues/84)) ([42bd94c](https://github.com/substrate-labs/substrate2/commit/42bd94c1f1d5e0b013a9b479bf100c68cf9de9a1))
* **layer-families:** implement layer families and clean up codegen ([#127](https://github.com/substrate-labs/substrate2/issues/127)) ([06f50b8](https://github.com/substrate-labs/substrate2/commit/06f50b8236ba40f405d7a5e20987a28e01f69f7c))
* **layout-io:** initial layout port API implementation ([#111](https://github.com/substrate-labs/substrate2/issues/111)) ([ecc8838](https://github.com/substrate-labs/substrate2/commit/ecc8838678c98f137aca6f4955d89ba350540b44))
* **layout-ports:** initial implementation of layout port traits ([3c0527a](https://github.com/substrate-labs/substrate2/commit/3c0527a749b2ef7f3b42e46ce66d9f9bed3ff947))
* **mos:** add standard 4-terminal MosIo ([8678716](https://github.com/substrate-labs/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **organization:** rename substrate to substrate_api, set up codegen crate ([#67](https://github.com/substrate-labs/substrate2/issues/67)) ([e07f099](https://github.com/substrate-labs/substrate2/commit/e07f09949551fd08e3f58b6ffb7d9a8c67b76ae9))
* **pdk:** add PDK trait and update context ([#68](https://github.com/substrate-labs/substrate2/issues/68)) ([a8fbd14](https://github.com/substrate-labs/substrate2/commit/a8fbd14a4b81e504c781e0656edce81853039afb))
* **pdks:** implement `supported_pdks` macro and add examples ([#72](https://github.com/substrate-labs/substrate2/issues/72)) ([5f4312f](https://github.com/substrate-labs/substrate2/commit/5f4312f5220ae6023d78d8f4e585032147195a75))
* **proc-macros:** add derive Block proc macro ([#151](https://github.com/substrate-labs/substrate2/issues/151)) ([e2c2f02](https://github.com/substrate-labs/substrate2/commit/e2c2f02771611ad4a79b3c9516fa1defabc20a66))
* **proc-macros:** allow missing docs on generated structs ([8678716](https://github.com/substrate-labs/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **proc-macros:** macros respect field and struct visibilities ([8678716](https://github.com/substrate-labs/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **proc-macros:** proc macros find substrate crate location ([#125](https://github.com/substrate-labs/substrate2/issues/125)) ([8678716](https://github.com/substrate-labs/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **schematic:** nested node and instance access ([#134](https://github.com/substrate-labs/substrate2/issues/134)) ([3d0e9ce](https://github.com/substrate-labs/substrate2/commit/3d0e9ce96b66072cd9b7982c582fa2d67ed8f406))
* **schematics:** implement node naming trees, with codegen ([#105](https://github.com/substrate-labs/substrate2/issues/105)) ([5ef8e4b](https://github.com/substrate-labs/substrate2/commit/5ef8e4b8cdd20a274d1a4dadda8e186bed004763))
* **schematics:** implement proc macro to derive AnalogIo ([#99](https://github.com/substrate-labs/substrate2/issues/99)) ([2320c99](https://github.com/substrate-labs/substrate2/commit/2320c99e9852d4698c5b336de0af7ebe7cc94204))


### Bug Fixes

* **deps:** update rust crate syn to v2 ([#79](https://github.com/substrate-labs/substrate2/issues/79)) ([eee3593](https://github.com/substrate-labs/substrate2/commit/eee35938247f2660c15b0165b6ba3d609d7091b8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate_api bumped from 0.0.0 to 0.1.0
  * dev-dependencies
    * substrate bumped from 0.0.0 to 0.1.0
