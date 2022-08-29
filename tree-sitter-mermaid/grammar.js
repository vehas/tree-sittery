module.exports = grammar({
    name: 'er_mermaid',
    rules: {
        er_digram: $ => seq('erDiagram', repeat($.statement)),
        statement: $ => choice(
            $.identifier,
            seq($.identifier, $.attributes),
        ),
        attributes: $ => seq(
            "{",
            repeat($.attribute),
            "}"
        ),
        attribute: $ => seq(
            field('type', $.identifier),
            field('name', $.identifier)
        ),
        identifier: _ => /([a-zA-Z_][0-9a-zA-Z_]*)/
    }
});