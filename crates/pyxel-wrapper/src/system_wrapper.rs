use std::process::exit;

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict};
use pyxel::{Pyxel, PyxelCallback};
#[cfg(not(target_os = "emscripten"))]
use sysinfo::{Pid, PidExt, System, SystemExt};

use crate::pyxel_singleton::{pyxel, set_pyxel_instance};

#[pyfunction]
#[pyo3(
    text_signature = "(width, height, *, title, fps, quit_key, display_scale, capture_scale, capture_sec)"
)]
fn init(
    py: Python,
    width: u32,
    height: u32,
    title: Option<&str>,
    fps: Option<u32>,
    quit_key: Option<pyxel::Key>,
    display_scale: Option<u32>,
    capture_scale: Option<u32>,
    capture_sec: Option<u32>,
) -> PyResult<()> {
    let locals = PyDict::new(py);
    locals.set_item("os", py.import("os")?)?;
    locals.set_item("inspect", py.import("inspect")?)?;
    py.run(
        "os.chdir(os.path.dirname(inspect.stack()[1].filename) or '.')",
        None,
        Some(locals),
    )?;
    set_pyxel_instance(pyxel::init(
        width,
        height,
        title,
        fps,
        quit_key,
        display_scale,
        capture_scale,
        capture_sec,
    ));
    Ok(())
}

#[pyfunction]
fn run(py: Python, update: &PyAny, draw: &PyAny) {
    struct PythonCallback<'a> {
        py: Python<'a>,
        update: &'a PyAny,
        draw: &'a PyAny,
    }

    impl<'a> PyxelCallback for PythonCallback<'a> {
        fn update(&mut self, _pyxel: &mut Pyxel) {
            if let Err(err) = self.update.call0() {
                err.print(self.py);
                exit(1);
            }
        }

        fn draw(&mut self, _pyxel: &mut Pyxel) {
            if let Err(err) = self.draw.call0() {
                err.print(self.py);
                exit(1);
            }
        }
    }

    pyxel().run(PythonCallback { py, update, draw });
}

#[pyfunction]
fn show() {
    pyxel().show();
}

#[pyfunction]
fn flip() {
    pyxel().flip();
}

#[pyfunction]
fn quit() {
    pyxel().quit();
}

#[pyfunction]
fn title(title: &str) {
    pyxel().title(title);
}

#[pyfunction]
fn icon(data: Vec<&str>, scale: u32, colkey: Option<pyxel::Color>) {
    pyxel().icon(&data, scale, colkey);
}

#[pyfunction]
fn fullscreen(full: bool) {
    pyxel().fullscreen(full);
}

#[pyfunction]
fn screen_mode(scr: u32) {
    pyxel().screen_mode(scr);
}

#[cfg(not(target_os = "emscripten"))]
#[pyfunction]
fn process_exists(pid: u32) -> bool {
    let system = System::new_all();
    system.process(Pid::from_u32(pid)).is_some()
}

pub fn add_system_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(show, m)?)?;
    m.add_function(wrap_pyfunction!(flip, m)?)?;
    m.add_function(wrap_pyfunction!(quit, m)?)?;
    m.add_function(wrap_pyfunction!(title, m)?)?;
    m.add_function(wrap_pyfunction!(icon, m)?)?;
    m.add_function(wrap_pyfunction!(fullscreen, m)?)?;
    m.add_function(wrap_pyfunction!(screen_mode, m)?)?;
    #[cfg(not(target_os = "emscripten"))]
    m.add_function(wrap_pyfunction!(process_exists, m)?)?;
    Ok(())
}
