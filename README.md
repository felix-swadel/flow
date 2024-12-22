# flow
A basic particle-based fluid simulator written in Rust with Bevy

# The Maths
The simulation is based fundamentally on the following approximation:

$$A(x) = \sum_i A_i \frac{m_i}{\rho_i}W(|x - x_i|)$$

Where $A$ is some property field, and $W$ is a kernel which defines how a particles 'influence' scales with distance, and we sum over each $i$-th particle.

Using this same equation we can estimate density: $\rho(x) = \sum_i m_i W(|x - x_i|)$.

For pressure, we use an approximation: $P(x) = P_m (\rho(x) - \rho_0)$, where $P_m$ is some pressure-density conversion factor and $\rho_0$ is some 'target density'.

From this we can compute the acceleration due to pressure force:

$$a(x) = \frac{1}{\rho(x)} \sum_i P_i \frac{m}{\rho_i} \nabla W(|x - x_i|)$$
