use pyo3::{PyResult, Python};

pub fn example() -> PyResult<()>
{
    Python::with_gil(|py| {
        Python::run_bound(py, r#"from PyMPDATA import Options
from PyMPDATA import ScalarField
from PyMPDATA import VectorField
from PyMPDATA import Stepper
from PyMPDATA import Solver
from PyMPDATA.boundary_conditions import Periodic
import matplotlib.pyplot as pyplot
import numpy as np

options = Options(n_iters=2)
nx, ny = 24, 24
Cx, Cy = -.5, -.25
halo = options.n_halo

xi, yi = np.indices((nx, ny), dtype=float)
advectee = ScalarField(
  data=np.exp(
    -(xi+.5-nx/2)**2 / (2*(nx/10)**2)
    -(yi+.5-ny/2)**2 / (2*(ny/10)**2)
  ),
  halo=halo,
  boundary_conditions=(Periodic(), Periodic())
)
advector = VectorField(
  data=(np.full((nx + 1, ny), Cx), np.full((nx, ny + 1), Cy)),
  halo=halo,
  boundary_conditions=(Periodic(), Periodic())
)
stepper = Stepper(options=options, n_dims=2)
stepper = Stepper(options=options, grid=(nx, ny))

solver = Solver(stepper=stepper, advectee=advectee, advector=advector)
state_0 = solver.advectee.get().copy()
solver.advance(n_steps=75)
state = solver.advectee.get()
def plot(psi, zlim, norm=None):
    xi, yi = np.indices(psi.shape)
    fig, ax = pyplot.subplots(subplot_kw={"projection": "3d"})
    pyplot.gca().plot_wireframe(
        xi+.5, yi+.5,
        psi, color='red', linewidth=.5
    )
    ax.set_zlim(zlim)
    for axis in (ax.xaxis, ax.yaxis, ax.zaxis):
        axis.pane.fill = False
        axis.pane.set_edgecolor('black')
        axis.pane.set_alpha(1)
    ax.grid(False)
    ax.set_zticks([])
    ax.set_xlabel('x/dx')
    ax.set_ylabel('y/dy')
    ax.set_proj_type('ortho')
    cnt = ax.contourf(xi+.5, yi+.5, psi, zdir='z', offset=-1, norm=norm)
    cbar = pyplot.colorbar(cnt, pad=.1, aspect=10, fraction=.04)
    return cbar.norm

zlim = (-1, 1)
norm = plot(state_0, zlim)
pyplot.savefig('readme_gauss_0.png')
plot(state, zlim, norm)
pyplot.savefig('readme_gauss.png')"#, None, None)?;
        Ok(())
    })
}