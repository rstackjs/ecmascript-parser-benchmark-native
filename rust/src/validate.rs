use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use swc_common::BytePos;
use swc_ecma_parser::{EsSyntax, Parser as SwcParser, StringInput, Syntax, TsSyntax};
use swc_next_allocator::Allocator as NextAllocator;
use swc_next_ecma_ast::Lang;
use swc_next_ecma_parser::{NoTokenParserConfig, Options, Parser as NextParser};
use swc_next_ecma_semantic::{AnalyzeOptions, SyntaxChecks, analyze};
fn main() {
    for path in std::env::args().skip(1) {
        let src = std::fs::read_to_string(&path).unwrap();
        let arena = Allocator::default();
        let ret = Parser::new(&arena, &src, SourceType::from_path(&path).unwrap()).parse();
        assert!(
            ret.diagnostics.is_empty(),
            "Oxc parse: {:?}",
            ret.diagnostics
        );
        assert!(!ret.program.body.is_empty());
        let sem = SemanticBuilder::new()
            .with_check_syntax_error(true)
            .build(&ret.program);
        assert!(
            sem.diagnostics.is_empty(),
            "Oxc semantic: {:?}",
            sem.diagnostics
        );
        println!(
            "{path}: Oxc statements={} symbols={} scopes={}",
            ret.program.body.len(),
            sem.semantic.scoping().symbols_len(),
            sem.semantic.scoping().scopes_len()
        );
        let syntax = if path.ends_with(".ts") {
            Syntax::Typescript(TsSyntax::default())
        } else {
            Syntax::Es(EsSyntax::default())
        };
        let mut parser = SwcParser::new(
            syntax,
            StringInput::new(&src, BytePos(0), BytePos(src.len() as u32)),
            None,
        );
        let ret = parser.parse_module().expect("SWC parse failure");
        let errors = parser.take_errors();
        if path.ends_with("lib.dom.d.ts") {
            assert_eq!(errors.len(), 2);
            assert!(
                errors
                    .iter()
                    .all(|error| format!("{error:?}").contains("EvalAndArgumentsInStrict"))
            );
            println!("{path}: SWC known module-mode diagnostics={errors:?}");
        } else {
            assert!(errors.is_empty(), "SWC recoverable errors: {errors:?}");
        }
        assert!(!ret.body.is_empty());
        println!("{path}: SWC statements={}", ret.body.len());
        let arena = NextAllocator::new();
        let ret = NextParser::init(
            &arena,
            &src,
            Options {
                lang: Lang::from_path(&path),
                ..Options::default()
            },
            NoTokenParserConfig,
        )
        .parse();
        assert!(
            ret.diagnostics.is_empty(),
            "SWC Next parse: {:?}",
            ret.diagnostics
        );
        let sem = analyze(
            &ret.ast,
            AnalyzeOptions {
                check_syntax: SyntaxChecks::all(),
                ..Default::default()
            },
        );
        assert!(
            sem.diagnostics.is_empty(),
            "SWC Next semantic: {:?}",
            sem.diagnostics
        );
        let model = sem.semantic.semantic();
        println!(
            "{path}: SWC Next scopes={} symbols={} references={}",
            model.iter_scopes().count(),
            model.iter_symbols().count(),
            model.iter_references().count()
        );
        assert!(model.iter_scopes().count() > 0);
    }
}
