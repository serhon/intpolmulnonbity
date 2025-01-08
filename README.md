# IntPolMulNonbity

Stochastic parallelised search for counter- and extreme examples to inequalities between "nonbities" of 2 polynomials with nonnegative coefficients and of their product, where a nonbity of a polynomial quantifies its "distinctiveness" from a polynomial whose coefficients are 0 and 1 only.

$$🤔 \quad \mathcal{N}(QR) \geqslant f \bigl( \mathcal{N}(Q), \mathcal{N}(R) \bigr), \quad f(u, v) = f(v, u), \quad f(u, v) = 0 \Leftrightarrow u = v = 0 \quad 🤨$$

Welcome to fringy ruminations about [unfair 0-1-polynomials conjecture](https://mathoverflow.net/questions/339137/why-do-polynomials-with-coefficients-0-1-like-to-have-only-factors-with-0-1), currently [the most voted unanswered question on MathOverflow](https://mathoverflow.net/unanswered) and an open problem since at least 1969 or even 1937.

**Attention:** hereinafter, "counterexample" means that to an inequality of given kind, *not* to this conjecture. To avoid ambiguity, we call the latter (whether it exists or not) an "unfair pair".

If you hypothesise an inequality such as the one above, go to [Results so far](#results-so-far) and check if it has been refuted, with corresponding counterexample. If so, be sad but happy that you have not spent time trying to prove a false statement. Since a single counterexample refutes multiple such hypotheses, even if exactly yours is not there, test it against all counterexamples, or try to tinker with them. Then grab the whole thing, [customise](#customisation) it to verify your inequality and run.

## `pol_mul_nonbity.html`

For the sake of convenience, we've also included this HTML5+JavaScript-based graphical interactive calculator of 2 polynomials' product. With it, you can observe "real-time" behaviour of various coefficients of the product as you change coefficients of the multipliers via sliders, adjust formulae by which scalar and vector nonbities are calculated, see their distributions, double-check results regarding inequalities between nonbities etc. Simply open it with a browser, modify as you need etc.

## Context

Let $Q$ and $R$ be polynomials over $\mathbb{R}_+$, that is,

$$Q(x) = \sum\limits_{j=0}^{d_Q} q_j x^j, \quad R(x) = \sum\limits_{l=0}^{d_R} r_l x^l$$

where $q_j \geqslant 0$, $q_{d_Q} > 0$, $r_l \geqslant 0$, $r_{d_R} > 0$, then $\deg Q = d_Q$ and $\deg R = d_R$.

For the sake of simplicity, we assume that $q_j$ and $r_l$ are defined $\forall j \in \mathbb{Z}, \forall l \in \mathbb{Z}$, and

$$\forall j \notin \{ 0, 1, \ldots, d_Q \}: q_j = 0, \quad \forall l \notin \{ 0, 1, \ldots, d_R \}: r_l = 0$$

Consider the product $P = QR$:

$$P(x) = \sum\limits_{i=0}^{d_P} p_i x^i, \quad p_i = \sum\limits_{k=0}^i q_k r_{i - k}$$

where $d_P = \deg P = d_Q + d_R$. Obviously, $p_i \geqslant 0$, and similarly we extend $\forall i \notin \{ 0, 1, \ldots, d_P \}$: $p_i = 0$.

Polynomials whose coefficients are only 0 and 1 are called *{0,1}*-polynomials or *Newman* polynomials.

**Unfair 0-1-polynomials conjecture:** *Assuming that $q_0 = r_0 = 1$, if $P$ is a {0,1}-polynomial, then $Q$ and $R$ are {0,1}-polynomials as well.*

An *unfair pair* would be $(Q, R)$ with $q_0 = r_0 = 1$ such that $Q$ and $R$ are not {0,1}-polynomials, but their product $P$ is.

Usually, instead of the constraint $q_0 = r_0 = 1$, there is a "symmetric" one: $q_{d_Q} = r_{d_R} = 1$, i.e. $Q$ and $R$ are *monic* polynomials.

One quickly sees the importance of constraints: without monicity, there is an unfair pair $x^2 = (\frac{1}{2}x) \cdot (2x)$ (or just $1 = \frac{1}{2} \cdot 2$), and without nonnegativity, $x^3 + 1 = (x + 1)(x^2 - x + 1)$ and $x^4 + 1 = (x^2 - \sqrt{2}x + 1)(x^2 + \sqrt{2}x + 1)$. Other, more implicit constraints, are critical too, for example the conjecture fails for infinite series: $\sum\limits_{i=0}^{\infty} x^i = \dfrac{1}{1-x} = \left( \dfrac{1}{\sqrt{1-x}} \right)^2 = \left( \sum\limits_{j=0}^{\infty} \dfrac{(2j)!}{4^j (j!)^2} x^j \right)^2$. (All these are from comments to [original MathOverflow question](https://mathoverflow.net/questions/339137/why-do-polynomials-with-coefficients-0-1-like-to-have-only-factors-with-0-1).)

Consider [polynomial long division](https://en.wikipedia.org/wiki/Polynomial_long_division) to see that in an unfair pair *both* $Q$ and $R$ have to be non-{0,1}. And $\forall q_j \leqslant 1, \forall r_l \leqslant 1$, otherwise e.g. $q_{j_0} > 1$ implies $p_{j_0} \geqslant q_{j_0} r_0 = q_{j_0} > 1$.

There is also a probabilistic interpretation about factorisation of (discrete) uniform distribution and equivalent one involving dices, which "a child understands". On the other hand, any essential progress seems to rely on things more sophisticated, such as [Gauss's lemma](https://en.wikipedia.org/wiki/Gauss%27s_lemma_(polynomials)).

You can find more detailed exposition, results, and references at [[GHI]](#refGHI), and, of course, see the original MO question and comments.

The problem being so famous, you easily imagine hundreds, if not thousands, of people working on it this very moment. (Aren't you one of them now.)

---

Our astray inquiry is different: it feels like there is a way from discrete to continuous here... *Something* that forbids unfairness may also forbid its "neighborhood", like a "repelling force" acting in its "vicinity".

Surely, this feeling may be wrong, especially if unfair polynomials *exist* after all. Or, behaviour of *something* turns out to be much more complex than the original problem.

Anyway, to turn feelings into reasonings, we need quantification. "Neighborhood", "vicinity" become such with respect to certain "distance" (not a *metric* in general case, though). For the lack of established term (or our laziness to find it), we call such characteristic a (vector) *nonbity* (since each 0/1 is a *bit* value), you are welcome to call it *anti-Newmanness* or *non-{0,1}ness* or *nonbinarity* or *Newmanlessity* or *unfairness*:

$$\mathcal{N}: \mathbb{R}_+[x] \mapsto \mathbb{R}_+$$

*It can be defined in many different ways.* Here we consider the "additive" one, that is,

$$\mathcal{N}(P) = \sum\limits_{i = 0}^{d_P} \eta (p_i)$$

where a (scalar) nonbity

$$\eta : \mathbb{R}_+ \mapsto \mathbb{R}_+, \quad \eta(0) = \eta(1) = 0, \quad \forall t \in (0; 1) \cup (1; +\infty) : \eta(t) > 0$$

To say nothing of $\eta$ itself, there are other ways — non-additive, non-real etc., and one of them is perhaps more appropriate for the problem at hand. Speaking of this problem, for an unfair pair we would clearly have

$$\mathcal{N}(P) = 0, \quad \mathcal{N}(Q) > 0, \quad \mathcal{N}(R) > 0$$

and thus $\mathcal{N}(Q) \mathcal{N}(R) > 0$, a contradiction with $\mathcal{N}(P) \geqslant C \mathcal{N}(Q) \mathcal{N}(R)$ (or $\mathcal{N}(P) \geqslant C_1 \mathcal{N}(Q) + C_2 \mathcal{N}(R)$, or $\mathcal{N}(P) \geqslant f \bigl( \mathcal{N}(Q), \mathcal{N}(R) \bigr)$, where $f(u,v) = f(v, u)$ and $f(u, v) = 0 \Leftrightarrow u = v = 0$) for some $C > 0$, **if such inequalities were established**.

This is one part of the motivation to, well, establish them. Another is that they would give more resolution to fairness phenomenon, reveal a "forbidden zone" for $P \sim (p_0, p_1, \ldots, p_{d_P})$ around $\mathcal{N}(P) = 0$, defined by $\mathcal{N}(Q)$ and $\mathcal{N}(R)$, provide sensing of the surface and the shape of that zone.

---

There are trivial inequalities of this sort that *do not* help to solve the original problem. Consider $\eta(t) = t |1 - t|$ and assume that $\forall q_j, r_l \in [0;1]$; also, let $\mu = \min\limits_i |1 - p_i|$. Then

$$\mathcal{N}(P) = \sum\limits_{i = 0}^{d_P} p_i |1 - p_i| \geqslant \mu \sum\limits_{i = 0}^{d_P} \sum\limits_{k=0}^i q_k r_{i-k} \geqslant \mu \sum\limits_{j=0}^{d_Q} \sum\limits_{l=0}^{d_R} q_k |1 - q_k| r_l |1 - r_l| = \mu \mathcal{N}(Q) \mathcal{N}(R)$$

But, of course, for an unfair pair $\mu = 0$ — no contradiction there.

By the way, this is the single *positive* result here.

---

Plenty of $\eta(t)$ exists that satisfy general requirements above:

$$\min \{ t, |1 - t| \}, \quad t|1 - t|, \quad t|\log t|, \quad t^2 (1 - t)^2, \quad |\sin (\pi t)|, \quad t(1 - t)^2, \quad t^2 |\log t|, \quad \ldots $$

And then there is the choice of $C$, which may depend on $q_j$ and $r_l$ (why not?):

$$\frac{1}{\sum\limits_{j=0}^{d_Q} q_j \cdot \sum\limits_{l=0}^{d_R} r_l}, \quad \frac{d_P}{\sum\limits_{j=0}^{d_Q} q_j \cdot \sum\limits_{l=0}^{d_R} r_l} \quad \frac{d_P}{d_Q d_R}, \quad \frac{1 + d_P}{(1 + d_Q)(1 + d_R)}, \quad \frac{1 + 2\max \{ d_Q, d_R \}}{(1 + \max \{ d_Q, d_R \})^2}, \quad \ldots$$

In the absence of understanding/vision/intuition of what is appropriate and what is not, we can try these one by one, in various combinations, rejecting those for which counterexamples have been found and concentrating on attempts to prove "suspicious" ones for which they have not been... yet.

IntPolMulNonbity performs that search stochastically, with optional gradient descent.

Hope it saves, not swallows, someone's time.

## $\mathcal{S}$ for Surplus

Any inequality has the form $\mathcal{L} \geqslant \mathcal{R}$, with e.g. $\mathcal{L} = \mathcal{N}(P)$ and $\mathcal{R} = C \mathcal{N}(Q) \mathcal{N}(R)$, which for $\mathcal{R} > 0$ can be rewritten as $\mathcal{L} / \mathcal{R} \geqslant 1$. *Surplus* is just that, $\mathcal{S} = \mathcal{L} / \mathcal{R}$, it provides a single number to watch: as soon as we find $(Q, R)$ such that $\mathcal{S} < 1$, the hypothesis at hand is refuted.

## Implementation

Straightforward, 500+ lines of a single source `main.rs` smaller than this README contain it all. We rely on [num_cpus](https://crates.io/crates/num_cpus) to get number of logical CPUs, [rayon](https://crates.io/crates/rayon) for parallelisation and [rand](https://crates.io/crates/rand) along with [rand_xoshiro](https://crates.io/crates/rand_xoshiro) for fast generation of sufficiently random numbers.

Since the mapping $P \mapsto \mathcal{N}(P)$ in all considered cases is continuous, the real coefficients of polynomials $Q$ and $R$ from any counterexample (such that $\mathcal{N}(QR) < f\bigl( \mathcal{N}(Q), \mathcal{N}(R) \bigr)$) can be replaced with their rational approximations retaining the inequality sign. Moreover, it appears that the search using, as coefficients, rationals with small denominators often finds counter- and extreme examples faster than the one using floating point numbers: the speed-up is quite noticeable, seconds vs. days, especially when "right" denominators are chosen. Uniformity of the majority of $\eta$ allows to make the next step, from rationals to integers (that's where "Int" in the name comes from). Also, the polynomials-tested-per-second rate is slightly faster thus. For these reasons the coefficients of polynomials in this implementation are `i64` instead of `f64`.

For example, scalar nonbity $\eta(t) = t|1 - t|$ for $t \in \mathbb{R}_+$ becomes $\eta(t) = t |D - t|$ for $t \in \mathbb{Z}_+$, $D \in \mathbb{N}$, and accordingly

$$\sum\limits_{i = 0}^{d_P} p_i |1 - p_i| \bigvee C \sum\limits_{j=0}^{d_Q} q_j|1 - q_j| \cdot \sum\limits_{l=0}^{d_R} r_l|1 - r_l|$$

where $q_j, r_l \in [0; 1]$ and $p_i \in [0; +\infty)$, becomes internally

$$\sum\limits_{i = 0}^{d_P} a_i |MN - a_i| \bigvee C \sum\limits_{j=0}^{d_Q} b_j|M - b_j| \cdot \sum\limits_{l=0}^{d_R} c_l|N - c_l|$$

where $b_j \in \{ 0, 1, \ldots, M \}$, $c_l \in \{ 0, 1, \ldots, N \}$, and $a_i = \sum\limits_{k = 0}^i b_k c_{i - k} \in \mathbb{Z}_+$.

There are some combinatorial interpretations of such expressions, e.g. selection of squares inside big squares on diagonals of even bigger square, but whether they make understanding easier or harder is vague... And the trick does not work for essentially nonlinear nonbities like $t|\log t|$ or $|\sin (\pi t)|$, which we leave as an exercise to the reader/coder.

There is not much *polynomial* arithmetic either, "polynomials" are represented simply as `Vec<i64>`, because we do not need their values at any points. Multiplication, of course, is merely a discrete convolution.

A `Searcher` instance is constructed with certain parameters (like range of polynomials' degrees). Then the specified number of instances of search runs in parallel in an endless loop. When a pair of polynomials is found such that its $\mathcal{S}$ is less than min of $\mathcal{S}$ values before (extreme example), and/or its $\mathcal{S} < 1$ (counterexample), this pair is printed along with the product, nonbities, and $\mathcal{S}$.

In turn, an instance of search randomly generates a pair of polynomials $(Q, R)$, then a discrete form of gradient descent minimises $\mathcal{S}$ in 2 stages:

**I.** All-coefficients-at-once: for each numerator $n_k$ of a coefficient of $Q$ and $R$ find its $\Delta_k \in \{ -1, 1 \}$ that decreases $\mathcal{S}$ the most (or rather increases it the less) and apply such $\Delta$-s simultaneously to their numerators: $n_k \leftarrow n_k + \Delta_k$. This stage continues until it becomes impossible to decrease $\mathcal{S}$.

**II.** One-by-one-coefficient: almost the same, but change the numerator of single coefficient at each step, the first one in the sequence $q_0, q_1, \ldots, q_{d_Q}, \quad r_0, r_1, \ldots, r_{d_R}$ whose decrement or increment strictly decreases $\mathcal{S}$.

There is a specified limit to total number of such steps. Note that at each step, $P = QR$ is recalculated, usually multiple times.

In the end, the "descended" $(Q, R)$ is used to compare its $\mathcal{S}$ with 1 and previous values as described above. And then it starts anew, and repeats for certain number of times, forming a *batch*.

See `Searcher::search()`.

## Quickstart

Assuming that [Rust toolchain has been installed](https://www.rust-lang.org/tools/install),

```shell
$ cd intpolmulnonbity
$ cargo build --release
$ cargo run --release
```

Wait... wait... then press `Ctrl+C` to break.

## Customisation

By default, we verify $\mathcal{N}(P) \stackrel{?}{\geqslant} C \mathcal{N}(Q) \mathcal{N}(R)$ hypothesis for $\eta(t) = t|1-t|$ (hence $\mathcal{N}(P) = \sum\limits_{i=0}^{d_P} p_i |1 - p_i|$) and $C = \dfrac{1 + 2\max \{ d_Q, d_R \}}{(1 + \max \{ d_Q, d_R \})^2}$ *without* the $q_0 = r_0 = 1$ constraint. No counterexamples have been found yet.

To change $\eta$, look into `nonbity()` function. Some alternatives are already there, commented.

To change $C$ and $\mathcal{S}$, look into `surplusity()`.

Almost all parameters controlling search in general and random generation in particular are defined at the beginning of `run_search_rand()`. Verbatim:

```rust
let deg_min: usize = 3;
let deg_max: usize = 17;
let denom_min: i64 = 4;
let denom_max: i64 = 8;
let grav_min: i8 = -4;
let grav_max: i8 = 1;
let lowest_is_unit = false;
let gradesc_max_steps: usize = 4000;
let batch_size: u128 = 1000;
let threshold: f64 = 1.0;
let n_searchers: usize = num_cpus::get();
```

Degree of polynomial is **U**niformly distributed in [`deg_min`; `deg_max`].

Denominator of polynomial's coefficients is U-distributed in [`denom_min`; `denom_max`].

[`grav_min`; `grav_max`] is the range of U-distribution of the "gravity" parameter, say $g$, that "skews" the distribution of a polynomial's coefficient $c$ toward 0 (when $g > 0$) or 1 (when $g < 0$). To be more precise,

$$c = \begin{cases}
\min \{ \xi_1, \xi_2, \ldots, \xi_g \}, & g > 0,\\
\max \{ \xi_1, \xi_2, \ldots, \xi_{-g}\}, & g < 0
\end{cases}$$

where independent $\xi_i \sim U(0, 1)$. In particular, the distribution of $c$ is uniform only if $g = 0$.

Set `lowest_is_unit = true` to require "back monicity", i.e. $q_0 = r_0 = 1$ (in terms of integer version, $b_0 = M$, $c_0 = N$).

`gradesc_max_steps` limits the number of gradient descent steps, stages I and II alltogether.

`batch_size` specifies the number of pairs $(Q, R)$ that each instance of search generates and tests at each iteration.

`threshold`: as soon as $\mathcal{S}$ is strictly less than this value (naturally 1), we have a counterexample.

Finally, `n_searchers` is the number of parallel instances of search running simultaneously. By default, it is the number of logical CPUs, but you can set it manually to e.g. avoid overheat.

...Feel free to modify the search algorithm itself, after all.

## Hints

From time to time, adjust ranges of polynomials' degrees, of denominators, and other parameters. For they restrict search to certain region of "config space", and while in one region counterexamples are scarce, in the next region they may be abundant (if there are any).

Keep track of extreme examples: for another inequality, they can be counterexamples.

The longer it runs, the more "precious" each new extreme example becomes, although the process [has no memory](https://en.wikipedia.org/wiki/Memorylessness#Without_memory).

Absence of evidence is not evidence of absence.

## Results so far

First, counterexamples; in no particular order.

We represent coefficients $q_j = b_j / D$ as a list of numerators $b_j$, then denominator $D$ follows in "( /$D$ )". Thus "$Q$: 1, 2, 0, 3, 1 ( /4 )" means $Q(x) = \frac{1}{4} + \frac{2}{4}x + \frac{3}{4}x^3 + \frac{1}{4}x^4$.

* $Q$: **2, 1, 0, 0, 2 ( /2 )** — $Q(x) = 1 + \frac{1}{2} x + x^4$
\
$R$: **4, 2, 3, 0, 0, 2, 0, 0, 4 ( /4 )** — $R(x) = 1 + \frac{1}{2}x + \frac{3}{4} x^2 + \frac{1}{2}x^5 + x^8$
\
(Then $P(x) = 1 + x + x^2 + \dfrac{3}{8}x^3 + x^4 + x^5 + x^6 + x^8 + x^9 + x^{12}$ — single $p_i \notin \{ 0, 1 \}$.)
\
Refutes at least:
\
$\mathcal{N}(P) \geqslant \min \{ \mathcal{N}(Q), \mathcal{N}(R) \}$
\
where
\
$\mathcal{N} = \sum \eta$
\
$\eta(t) = \eta(1 - t)$ ($\eta(t)$ is symmetrical w.r.t. $t = \frac{1}{2}$) and $\eta(t)$ increases on $[0; \frac{1}{2}]$ (accordingly decreases on $[\frac{1}{2}; 1]$).

Although $\min$ does not satisfy "$f(u, v) = 0 \Leftrightarrow u=v=0$", we've mentioned that in unfair pair both $Q$ and $R$ must have $\mathcal{N} > 0$.

Its variant, simpler but violates $p_i \leqslant 1$ requirement:

* $Q$: **2, 0, 1, 2 ( /2 )**
\
$R$: **4, 0, 2, 0, 3, 2, 4 ( /4 )**
\
(Then in $P(x)$ only $p_6 = \frac{11}{8}$, others are either 0 or 1.)

* $Q$: **2, 1, 1 ( /2 )**
\
$R$: **4, 2, 1 ( /4 )**
\
Refutes at least:
\
$\mathcal{N}(P) \geqslant \min \{ \mathcal{N}(Q), \mathcal{N}(R) \}$
\
where
\
$\mathcal{N} = \sum \eta$
\
$\eta(t) = t|\log t|$

* $Q$: **2, 2 ( /2 )**
\
$R$: **2, 0, 1, 1, ..., 1, 0, 2 ( /2 )** — $d$ ones between 2 zeros
\
(Then $P(x) = 1 + x + \dfrac{1}{2}x^2 + x^3 + x^4 + \ldots + x^{d+1} + \dfrac{1}{2}x^{d+2} + x^{d+3} + x^{d+4}$)
\
Refutes at least:
\
$\mathcal{N}(P) \geqslant \gamma \max \{ \mathcal{N}(Q), \mathcal{N}(R) \}$
\
where
\
$\mathcal{N} = \sum \eta$
\
$\eta (t)$ is... arbitrary, satisfying general requirements for scalar nonbity.

For any fixed $\gamma > 0$: since the number of $r_l = \frac{1}{2}$ in $R$ is $d$, as $d \rightarrow \infty$, $\mathcal{N}(R) = d\eta(\frac{1}{2}) \rightarrow +\infty$, while in $Q$ there are none, and in $P$ there are 2, so $\mathcal{L} = \mathcal{N}(P) \equiv 2 \eta(\frac{1}{2})$ and $\mathcal{R} = \gamma \mathcal{N}(R)$; clearly, $\mathcal{S} \xrightarrow[d \rightarrow \infty]{} 0$.

* $Q$: **2, 1 ( /2 )**
\
$R$: **8, 4, 6, 5 ( /8 )**
\
(Then $P(x) = 1 + x + x^2 + x^3 + \frac{5}{16}x^4$.)
\
Refutes at least:
\
$\mathcal{N}(P) \geqslant \max \{ \dfrac{\mathcal{N}(Q)}{\sum\limits_{j=0}^{d_Q} q_j}, \dfrac{\mathcal{N}(R)}{\sum\limits_{l=0}^{d_R} r_l} \}$
\
where
\
$\mathcal{N} = \sum \eta$
\
$\eta (t) = t|1-t|$

* $Q$: **1, 1, 1, 1 ( /2 )**
\
$R$: **1, 1, 1, 1, 1, 1 ( /2 )**
\
Refutes at least:
\
$\mathcal{N}(P) \geqslant \dfrac{1 + 2\max \{ d_Q, d_R \}}{(1 + \max \{ d_Q, d_R \})^2} \mathcal{N}(Q) \mathcal{N}(R)$
\
where
\
$\mathcal{N} = \sum \eta$
\
$\eta (t) = \min \{ t, |1-t| \}$

and 2 of its generalisations:

* $Q$: **1, 1, 1, 1 ( /2 )** — 4 ones
\
$R$: **1, 1, ..., 1 ( /2 )** — all ones
\
(Then $P(x) = \frac{1}{4} + \frac{1}{2}x + \frac{3}{4}x^2 + x^3 + x^4 + \ldots + x^{d_R + 1} + \frac{3}{4}x^{d_R + 2} + \frac{1}{2}x^{d_R + 3} + \frac{1}{4}x^{d_R + 4}$.)
\
Refutes at least:
\
$\mathcal{N}(P) \geqslant \gamma \dfrac{1 + d_P}{(1 + d_Q)(1 + d_R)} \mathcal{N}(Q) \mathcal{N}(R)$
\
where
\
$\mathcal{N} = \sum \eta$
\
$\eta (t)$ is arbitrary scalar nonbity.

For any fixed $\gamma > 0$ we have $C = \gamma \dfrac{5 + d_R}{5(1 + d_R)} \geqslant \gamma / 5$, $\mathcal{N}(Q) \equiv \mathrm{const} > 0$, $\mathcal{N}(P) \equiv \mathrm{const} > 0$, $\mathcal{N}(R) \xrightarrow[d_R \rightarrow \infty]{} +\infty$, so $\mathcal{S} \xrightarrow[d_R \rightarrow \infty]{} 0$.

* $Q$: **1, 1, 1, 1, 0, 0, ..., 0, 1, 1, 1, 1 ( /2 )** — 4 ones, zeros, 4 ones
\
$R$: **1, 1, ..., 1 ( /2 )** — all ones

All these have $\mathcal{N} = \sum \eta$. Finally, there is one for $\mathcal{N} = \max \eta$, and it is sparse:

* $Q$: $q_K = \frac{1}{2}$, $q_{K - k_m} = \frac{1}{8}$ for $m = \overline{1, L}$, all other $q_j = 0$
\
$R$: $r_0 = \frac{1}{2}$, $r_{k_m} = \frac{1}{8}$, all other $r_l = 0$
\
where $K$ is large enough and $\{ k_m \}_{m=1}^L$ is an increasing sequence such that pairwise sums are all distinct. The simplest one is perhaps $k_m = 3^m$, but there are others, "minimal" of them being, by definition, [A025582](https://oeis.org/A025582).
\
Refutes at least:
\
$\mathcal{N}(P) \geqslant \mathcal{N}(Q) \mathcal{N}(R)$
\
where
\
$\mathcal{N} = \max \eta$
\
$\eta (t) = t |1-t|$

In fact, certain denser sequence suffices, but even it has $L = 44$ and $d_R \approx 800$, which is too much for the search method at hand (think centuries of runtime). So, it was found by means of... well, thinking rather than random searching.

---

Second, **hypothesised** inequalities still standing.

* $\mathcal{N}(P) \stackrel{?}{\geqslant} \dfrac{1 + 2\max \{ d_Q, d_R \}}{(1 + \max \{ d_Q, d_R \})^2} \mathcal{N}(Q) \mathcal{N}(R)$
\
where
\
$\mathcal{N} = \sum \eta$
\
$\eta (t) = t |1-t|$, also $\eta(t) = t^2 (1 - t)^2$

$C$ here is sharp from above in the sense that $\forall \delta > 0$ $C + \delta$ fails: consider $Q = R = \alpha \in (0;1]$. $d_Q = d_R = d_P = 0 \Rightarrow C = 1$, $\mathcal{N}(Q) = \mathcal{N}(R) = \alpha (1 - \alpha)$, $\mathcal{N}(P) = \alpha^2 (1 - \alpha^2)$, therefore

$$\mathcal{S} = \frac{\mathcal{N}(P)}{C \mathcal{N}(Q) \mathcal{N}(Q)} = \frac{1 + \alpha}{1 - \alpha} \xrightarrow[\alpha \rightarrow 0]{} 1$$

Clearly, $\inf\limits_{\mathcal{R} > 0} \mathcal{S} \leqslant 1$, but whether there is actually equality here (then $C$ would be simply sharp), we do not know. Whether $C$ can be improved for $d_Q, d_R > 0$ is unknown to us too (current $\inf$ is $\frac{5}{4}$, attained for the penultimate counterexample above).

To give more meaning to this $C$, we rewrite the inequality as

$$\frac{\mathcal{N}(P)}{1 + 2d} \stackrel{?}{\geqslant} \frac{\mathcal{N}(Q)}{1 + d} \cdot \frac{\mathcal{N}(R)}{1 + d}$$

where $d = \max \{ \deg Q, \deg R \}$: each ratio is the average scalar nonbity of corresponding polynomial, perhaps extended with zero terms beyond its highest term.

$\max$ is somehow important: $C = \dfrac{1 + d_P}{(1 + d_Q)(1 + d_R)}$ does not work. On the other hand, $\max$ is implied by $d_Q = d_R$.

$\eta(t)$ is important too: $\eta(t) = \min \{ t, |1 - t| \}$ does not work either...

A refutation not only of this hypothesis, but of its entire *class* may lie somewhere along the lines of 1st counterexample above.

* $\mathcal{N}(P) \stackrel{?}{\geqslant} \dfrac{\mathcal{N}(Q)}{\sum\limits_{j=0}^{d_Q} q_j} \cdot \dfrac{\mathcal{N}(R)}{\sum\limits_{l=0}^{d_R} r_l}$
\
where
\
$\mathcal{N} = \sum \eta$
\
$\eta (t) = \min \{ t, |1 - t| \}$, also $\eta (t) = t |1-t|$, $\eta(t) = t^2 (1 - t)^2$
\
with constraint $q_0 = r_0 = 1$

* $\mathcal{N}(P) \stackrel{?}{\geqslant} \min \{ \mathcal{N}(Q), \mathcal{N}(R) \}$
\
where
\
$\mathcal{N} = \sum \eta$
\
$\eta(t) = t \log t$ ($\eta(0) := 0$ by continuity) — "entropical"
\
with constraints $q_0 = r_0 = 1$, $q_{d_Q} = r_{d_R} = 1$, and $\forall p_i \leqslant 1$.

## Blind-spots

The last counterexample should confirm your suspicion that **random search as implemented here entirely misses huge classes of possibilities** if they start to appear only after polynomials' degrees reach 1000 or even 100. Please keep in mind the "surprise" of [105th cyclotomic polynomial](https://en.wikipedia.org/wiki/Cyclotomic_polynomial#Examples).

## References

<a id="refASG">[ASG]</a> Asgarli S., Hartglass M., Ostrov D., Walden B. "A fair shake: how close can the sum of $n$-sided dice be to a uniform distribution?", arXiv, 2023. [arXiv:2304.08501](https://arxiv.org/abs/2304.08501)

<a id="refBAR">[BAR]</a> Barbeau E.J. *Polynomials.* Springer-Verlag, 1989.

<a id="refGHI">[GHI]</a> Ghidelli L. "Progress on the unfair 0-1-polynomials conjecture using linear recurrences and numerical analysis", *arXiv*, 2021. [arXiv:2209.09843](https://arxiv.org/abs/2209.09843)

<a id="refHAR">[HAR]</a> Hare K.G. "Computational progress on the unfair 0-1 polynomial conjecture", *arXiv*, 2023. [arXiv:2307.07363](https://arxiv.org/abs/2307.07363)

<a id="refKRA">[KRA]</a> Krasner M., Ranulac B. "Sur une propriété des polynômes de la division du cercle", *CR Acad. Sci. Paris*, 1937, 240:397–399.

<a id="refLEW">[LEW]</a> Lewis T. "The Factorisation of the rectangular distribution", *J. App. Prob.*, 1967, 4(3):529–542. [doi:10.2307/3212219](https://doi.org/10.2307/3212219)

<a id="refMOR">[MOR]</a> Morrison I. "Sacks of dice with fair totals", *Amer. Math. Monthly*, 2018, 125(7):579–592. [doi:10.1080/00029890.2018.1473699](https://doi.org/10.1080/00029890.2018.1473699), [arXiv:1411.2272](https://arxiv.org/abs/1411.2272)

<a id="refPRA">[PRA]</a> Prasolov V.V. *Polynomials.* Transl. by Leites D. Springer, 2004.


