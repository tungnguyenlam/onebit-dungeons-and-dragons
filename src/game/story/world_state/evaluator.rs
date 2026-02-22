use super::types::WorldState;

impl WorldState {

    // -----------------------------------------------------------------------
    // Condition evaluator
    // -----------------------------------------------------------------------

    /// Evaluate a condition string against the current state.
    ///
    /// Returns `true` for an empty condition (unconditional).
    ///
    /// `||` is split first (lowest precedence); each branch is then split on
    /// `&&`.  Individual predicates are parsed and evaluated.
    pub fn evaluate(&self, condition: &str) -> bool {
        let cond = condition.trim();
        if cond.is_empty() {
            return true;
        }
        // Split on `||` (lowest precedence).
        let or_branches: Vec<&str> = cond.split("||").collect();
        for branch in or_branches {
            // Each branch must satisfy ALL of its `&&` sub-expressions.
            if branch
                .split("&&")
                .map(str::trim)
                .all(|pred| self.eval_predicate(pred))
            {
                return true;
            }
        }
        false
    }

    /// Evaluate a single predicate atom (no `&&` or `||`).
    fn eval_predicate(&self, pred: &str) -> bool {
        let pred = pred.trim();

        // --- `not flag:key` ---
        if let Some(rest) = pred.strip_prefix("not flag:") {
            return !self.flag(rest.trim());
        }

        // --- `flag:key` ---
        if let Some(rest) = pred.strip_prefix("flag:") {
            return self.flag(rest.trim());
        }

        // --- `counter:key OP N` ---
        if let Some(rest) = pred.strip_prefix("counter:") {
            return self.eval_counter_pred(rest.trim());
        }

        // Unknown predicates are treated as false (fail-safe).
        false
    }

    /// Parse and evaluate `"key OP N"` where OP ∈ { >=, >, <=, <, == }.
    fn eval_counter_pred(&self, expr: &str) -> bool {
        // Try each operator longest first to avoid prefix ambiguity.
        for op in &[">=", "<=", "==", ">", "<"] {
            if let Some(pos) = expr.find(op) {
                let key = expr[..pos].trim();
                let n_str = expr[pos + op.len()..].trim();
                if let Ok(n) = n_str.parse::<i32>() {
                    let val = self.counter(key);
                    return match *op {
                        ">=" => val >= n,
                        "<=" => val <= n,
                        "==" => val == n,
                        ">" => val > n,
                        "<" => val < n,
                        _ => false,
                    };
                }
            }
        }
        false
    }
}