fn main() {
    // ANCHOR: here
    enum Status {
        Значение(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Значение).collect();
    // ANCHOR_END: here
}
