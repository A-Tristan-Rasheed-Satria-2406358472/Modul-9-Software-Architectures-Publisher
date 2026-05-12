# Publisher Reflection Notes

## (1) How much data your publisher program will send to the message broker in one run?

Kalau lihat isi programnya, publisher memanggil `publish_event` sebanyak 5 kali. Artinya, dalam satu kali program dijalankan, publisher akan mengirim 5 message ke message broker.

Bagian kodenya kurang lebih seperti ini:

```rust
let _ = p.publish_event(... user_id: "1".to_owned(), ...);
let _ = p.publish_event(... user_id: "2".to_owned(), ...);
let _ = p.publish_event(... user_id: "3".to_owned(), ...);
let _ = p.publish_event(... user_id: "4".to_owned(), ...);
let _ = p.publish_event(... user_id: "5".to_owned(), ...);
```

Kalau dijelasin:

- Setiap pemanggilan `publish_event` mengirim satu event
- Satu event berisi struct `UserCreatedEventMessage`
- Di dalam struct itu ada dua field, yaitu `user_id` dan `user_name`
- Karena fungsi dipanggil 5 kali, total event yang terkirim dalam satu run adalah 5 data

Jadi jawaban paling langsungnya adalah: publisher mengirim 5 message atau 5 event dalam sekali eksekusi program.

Insight dari bagian ini:

- Satu kali `publish_event` berarti satu message dikirim ke broker
- Jumlah data yang dikirim bisa dihitung langsung dari jumlah pemanggilan fungsi
- Setiap message membawa payload yang isinya `user_id` dan `user_name`

## (2) The url of: “amqp://guest:guest@localhost:5672” is the same as in the subscriber program, what does it mean?

URL `amqp://guest:guest@localhost:5672` adalah alamat koneksi yang dipakai publisher untuk terhubung ke message broker.

Kalau dipecah:

- `amqp://` adalah protokol yang dipakai, yaitu `Advanced Message Queuing Protocol`
- `guest` pertama adalah username
- `guest` kedua adalah password
- `localhost` berarti broker berjalan di komputer yang sama
- `5672` adalah port default AMQP

Jadi maknanya, program publisher mencoba tersambung ke broker AMQP lokal memakai akun default `guest` dengan password `guest`, lewat port `5672`.

Karena subscriber juga memakai URL yang sama, artinya publisher dan subscriber terhubung ke broker yang sama. Itu yang bikin message yang dikirim publisher bisa diterima oleh subscriber.

- Publisher dan subscriber harus mengarah ke broker yang sama supaya bisa saling terhubung lewat message
- URL koneksi berisi protokol, username, password, host, dan port
- `localhost:5672` menunjukkan service broker berjalan secara lokal di port standar AMQP

# Running RabbitMQ as message brokert

![Running RabbitMQ as message brokert](public/image/rabbit.png)

# Sending and processing event

![Sending and processing event](public/image/rabbit-run.png)

# Monitoring chart based on publisher

![Monitoring chart based on publisher](public/image/monitoring.png)

# Cloud Experiment (Bonus)

Pada versi cloud, publisher dan subscriber tidak lagi memakai `localhost`, tetapi memakai host/IP VM tempat RabbitMQ berjalan, misalnya:

`amqp://guest:guest@<PUBLIC_IP_OR_DNS>:5672`

Supaya publisher bisa connect dari luar mesin broker, port AMQP harus dibuka di firewall/security group:

- `5672` untuk koneksi AMQP client (publisher/subscriber)
- `15672` opsional, untuk akses RabbitMQ Management UI dari browser

Refleksi “Make it works” di cloud:

- Saat port belum dibuka, error yang muncul biasanya `ConnectionRefused` atau timeout.
- Setelah port dibuka dan service RabbitMQ aktif, publisher langsung bisa push message ke queue.
- Dari sisi publisher, burst traffic tetap bisa terkirim cepat karena publisher hanya enqueue ke broker, bukan menunggu subscriber selesai proses.
