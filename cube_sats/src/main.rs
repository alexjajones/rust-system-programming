#[derive(Debug)]
struct GroundStation;

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }

    fn send(&self, message: Message, mail_box: &mut MailBox) {
        mail_box.post(message);
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![0, 1, 2]
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn recv(&self, mail_box: &mut MailBox) -> Option<Message> {
        mail_box.deliver(&self)
    }
}

#[derive(Debug)]
struct MailBox {
    messages: Vec<Message>,
}

impl MailBox {
    fn post(&mut self, message: Message) {
        self.messages.push(message)
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                return Some(self.messages.remove(i));
            }
        }
        None
    }
}

#[derive(Debug)]
struct Message {
    content: String,
    to: u64,
}

fn main() {
    let ground_station = GroundStation {};
    let mut mail_box = MailBox { messages: vec![] };

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let message = Message {
            content: "hello".to_string(),
            to: sat_id,
        };

        ground_station.send(message, &mut mail_box);
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = ground_station.connect(sat_id);
        let msg = sat.recv(&mut mail_box);

        println!("{:?} {:?}", sat_id, msg);
    }
}
