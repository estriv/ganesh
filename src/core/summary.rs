use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{Bound, Bounds};

/// A struct that holds the results of a minimization run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    /// The bounds of the parameters. This is `None` if no bounds were set.
    pub bounds: Option<Bounds>,
    /// The names of the parameters. This is `None` if no names were set.
    pub parameter_names: Option<Vec<String>>,
    /// A message that can be set by minimization algorithms.
    pub message: String,
    /// The initial parameters of the minimization.
    pub x0: Vec<f64>,
    /// The current parameters of the minimization.
    pub x: Vec<f64>,
    /// The standard deviations of the parameters at the end of the fit.
    pub std: Vec<f64>,
    /// The current value of the minimization problem function at [`Summary::x`].
    pub fx: f64,
    /// The number of function evaluations.
    pub cost_evals: usize,
    /// The number of gradient evaluations.
    pub gradient_evals: usize,
    /// Flag that says whether or not the fit is in a converged state.
    pub converged: bool,
}

impl Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use tabled::{
            builder::Builder,
            settings::{
                object::Row,
                style::{BorderSpanCorrection, HorizontalLine},
                Alignment, Color, Padding, Span, Style, Theme,
            },
        };
        let mut builder = Builder::default();
        builder.push_record(["FIT RESULTS"]);
        builder.push_record(["Status", "f(x)", "", "#f(x)", "", "#∇f(x)", ""]);
        builder.push_record([
            self.converged
                .then_some("Converged")
                .unwrap_or("Invalid Minimum"),
            &format!("{:.5}", self.fx),
            "",
            &format!("{:.5}", self.cost_evals),
            "",
            &format!("{:.5}", self.gradient_evals),
            "",
        ]);
        builder.push_record(["Message", &self.message]);

        let names = self
            .parameter_names
            .clone()
            .unwrap_or(
                vec![""; self.x.len()]
                    .into_iter()
                    .enumerate()
                    .map(|(i, _)| format!("x_{}", i))
                    .collect::<Vec<_>>(),
            )
            .into_iter();
        let bounds = self
            .bounds
            .clone()
            .map(|b| b.into_inner())
            .unwrap_or(vec![Bound::NoBound; self.x.len()])
            .into_iter();

        builder.push_record(["Parameter", "", "", "", "Bound", "", "At Limit?"]);

        builder.push_record(["", "=", "σ", "0", "-", "+", ""]);
        for ((((v, v0), e), b), n) in self
            .x
            .iter()
            .zip(&self.x0)
            .zip(&self.std)
            .zip(bounds)
            .zip(names)
        {
            builder.push_record([
                &n,
                &format!("{:.5}", v),
                &format!("{:.5}", e),
                &format!("{:.5}", v0),
                &format!("{:.5}", b.lower()),
                &format!("{:.5}", b.upper()),
                b.at_bound(*v).then_some("Yes").unwrap_or("No"),
            ]);
        }
        let mut table = builder.build();
        let mut style = Theme::from_style(Style::rounded().remove_horizontals());
        style.insert_horizontal_line(1, HorizontalLine::inherit(Style::modern()));
        style.insert_horizontal_line(2, HorizontalLine::inherit(Style::modern()));
        style.insert_horizontal_line(3, HorizontalLine::inherit(Style::modern()));
        style.insert_horizontal_line(4, HorizontalLine::inherit(Style::modern()));
        style.insert_horizontal_line(5, HorizontalLine::inherit(Style::modern()));
        style.insert_horizontal_line(6, HorizontalLine::inherit(Style::modern()));

        table
            .with(style)
            .modify(
                Row::from(0),
                (Padding::new(1, 1, 1, 1), Alignment::center(), Color::BOLD),
            )
            .modify((0, 0), Span::column(7))
            .modify(Row::from(1), Color::BOLD)
            .modify((1, 1), Span::column(2))
            .modify((1, 3), Span::column(2))
            .modify((1, 5), Span::column(2))
            .modify((2, 1), Span::column(2))
            .modify((2, 3), Span::column(2))
            .modify((2, 5), Span::column(2))
            .modify(Row::from(3), Padding::new(1, 1, 1, 1))
            .modify((3, 0), Color::BOLD)
            .modify((3, 1), Span::column(6))
            .modify(Row::from(4), Color::BOLD)
            .modify((4, 0), Span::column(4))
            .modify((4, 4), Span::column(2))
            .modify(Row::from(5), Color::BOLD)
            .with(BorderSpanCorrection);

        f.write_str(&table.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_minimization_result() {
        use super::*;
        let result = Summary {
            bounds: None,
            parameter_names: None,
            message: "Success".to_string(),
            x0: vec![1.0, 2.0, 3.0],
            x: vec![1.0, 2.0, 3.0],
            std: vec![0.1, 0.2, 0.3],
            fx: 3.0,
            cost_evals: 10,
            gradient_evals: 5,
            converged: true,
        };
        println!("{}", result);
    }
}
