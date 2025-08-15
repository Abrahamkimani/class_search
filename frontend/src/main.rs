use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="container">
            <h1>{ "Class Search Attendance" }</h1>
            <ClassForm />
            <StudentForm />
            <AttendanceForm />
            <Analytics />
        </div>
    }
}

#[function_component(ClassForm)]
fn class_form() -> Html {
    html! {
        <form>
            <h2>{ "Register Class" }</h2>
            <input placeholder="UUID" />
            <input placeholder="Name" />
            <input placeholder="Date" />
            <button type="submit">{ "Register" }</button>
        </form>
    }
}

#[function_component(StudentForm)]
fn student_form() -> Html {
    html! {
        <form>
            <h2>{ "Register Student" }</h2>
            <input placeholder="Name" />
            <input placeholder="Email" />
            <button type="submit">{ "Register" }</button>
        </form>
    }
}

#[function_component(AttendanceForm)]
fn attendance_form() -> Html {
    html! {
        <form>
            <h2>{ "Mark Attendance" }</h2>
            <input placeholder="Class UUID" />
            <button type="submit">{ "Mark" }</button>
        </form>
    }
}

#[function_component(Analytics)]
fn analytics() -> Html {
    html! {
        <div>
            <h2>{ "Attendance Analytics" }</h2>
            <p>{ "Total attendance and per-class stats will appear here." }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
