// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AttributeMacroUsage } from "./AttributeMacroUsage";
import type { DeriveMacroUsage } from "./DeriveMacroUsage";

export type MacroAnalyzis = { attribute_macro_definition_count: number, declarative_macro_definition_count: number, procedural_macro_definition_count: number, derive_macro_definition_count: number, derive_macro_usage: DeriveMacroUsage, attribute_macro_invocation_count: number, attribute_macro_usage: AttributeMacroUsage, macro_invocation_count: number, };
