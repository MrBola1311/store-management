use std::collections::HashMap;
use std::io::{self, Write};

// Kullanıcı rolleri
enum KullaniciRolu {
    Yonetici,
    Calisan,
}

// Kullanıcı yapısı
struct Kullanici {
    kullanici_adi: String,
    sifre: String, // Şifreleme yapmıyoruz, basit metin olarak saklıyoruz
    rol: KullaniciRolu,
}

// Ürün yapısı
struct Urun {
    id: u32,
    ad: String,
    aciklama: String,
    fiyat: f64,
    miktar: u32,
    maliyet: f64,
}

// Satış yapısı
struct Satis {
    id: u32,
    urun_id: u32,
    miktar: u32,
    satis_fiyati: f64,
    toplam_fiyat: f64,
    kar: f64,
    tarih: String, // Basitlik için tarih string olarak saklanıyor
    kullanici_id: u32,
}

// Alım yapısı
struct Alim {
    id: u32,
    urun_id: u32,
    miktar: u32,
    alim_fiyati: f64,
    toplam_maliyet: f64,
    tarih: String, // Basitlik için tarih string olarak saklanıyor
    kullanici_id: u32,
}

// Mağaza yapısı - tüm verileri içerir
struct Magaza {
    urunler: HashMap<u32, Urun>,
    satislar: HashMap<u32, Satis>,
    alimlar: HashMap<u32, Alim>,
    kullanicilar: HashMap<u32, Kullanici>,
    sonraki_urun_id: u32,
    sonraki_satis_id: u32,
    sonraki_alim_id: u32,
    sonraki_kullanici_id: u32,
}

impl Magaza {
    fn new() -> Self {
        // Başlangıç mağaza yapısını oluştur
        let mut magaza = Magaza {
            urunler: HashMap::new(),
            satislar: HashMap::new(),
            alimlar: HashMap::new(),
            kullanicilar: HashMap::new(),
            sonraki_urun_id: 1,
            sonraki_satis_id: 1,
            sonraki_alim_id: 1,
            sonraki_kullanici_id: 1,
        };

        // Varsayılan yönetici hesabı oluştur
        magaza.kullanici_olustur("admin".to_string(), "admin".to_string(), KullaniciRolu::Yonetici);
        
        magaza
    }

    fn kullanici_olustur(&mut self, kullanici_adi: String, sifre: String, rol: KullaniciRolu) -> u32 {
        // Kullanıcı adının benzersiz olup olmadığını kontrol et
        for kullanici in self.kullanicilar.values() {
            if kullanici.kullanici_adi == kullanici_adi {
                println!("Bu kullanıcı adı zaten kullanılıyor!");
                return 0;
            }
        }

        let id = self.sonraki_kullanici_id;
        self.kullanicilar.insert(id, Kullanici {
            kullanici_adi,
            sifre,
            rol,
        });

        self.sonraki_kullanici_id += 1;
        id
    }

    fn kullanici_girisi(&self, kullanici_adi: &str, sifre: &str) -> Option<u32> {
        for (id, kullanici) in &self.kullanicilar {
            if kullanici.kullanici_adi == kullanici_adi && kullanici.sifre == sifre {
                return Some(*id);
            }
        }
        None
    }

    fn yonetici_mi(&self, kullanici_id: u32) -> bool {
        if let Some(kullanici) = self.kullanicilar.get(&kullanici_id) {
            match kullanici.rol {
                KullaniciRolu::Yonetici => true,
                KullaniciRolu::Calisan => false,
            }
        } else {
            false
        }
    }

    fn urun_ekle(&mut self, ad: String, aciklama: String, fiyat: f64, miktar: u32, maliyet: f64) -> u32 {
        let id = self.sonraki_urun_id;
        self.urunler.insert(id, Urun {
            id,
            ad,
            aciklama,
            fiyat,
            miktar,
            maliyet,
        });

        self.sonraki_urun_id += 1;
        id
    }

    fn urun_guncelle(&mut self, id: u32, ad: String, aciklama: String, fiyat: f64, miktar: u32, maliyet: f64) -> bool {
        if let Some(urun) = self.urunler.get_mut(&id) {
            urun.ad = ad;
            urun.aciklama = aciklama;
            urun.fiyat = fiyat;
            urun.miktar = miktar;
            urun.maliyet = maliyet;
            true
        } else {
            false
        }
    }

    fn urun_sil(&mut self, id: u32) -> bool {
        if self.urunler.contains_key(&id) {
            self.urunler.remove(&id);
            true
        } else {
            false
        }
    }

    fn satis_yap(&mut self, urun_id: u32, miktar: u32, kullanici_id: u32) -> Result<u32, String> {
        if let Some(urun) = self.urunler.get_mut(&urun_id) {
            if urun.miktar < miktar {
                return Err(format!("Yetersiz stok! Mevcut: {}", urun.miktar));
            }

            let satis_fiyati = urun.fiyat;
            let toplam_fiyat = satis_fiyati * miktar as f64;
            let kar = (satis_fiyati - urun.maliyet) * miktar as f64;
            let tarih = chrono_string();

            let id = self.sonraki_satis_id;
            self.satislar.insert(id, Satis {
                id,
                urun_id,
                miktar,
                satis_fiyati,
                toplam_fiyat,
                kar,
                tarih,
                kullanici_id,
            });

            urun.miktar -= miktar;
            self.sonraki_satis_id += 1;

            Ok(id)
        } else {
            Err("Ürün bulunamadı!".to_string())
        }
    }

    fn alim_yap(&mut self, urun_id: u32, miktar: u32, alim_fiyati: f64, kullanici_id: u32) -> Result<u32, String> {
        if let Some(urun) = self.urunler.get_mut(&urun_id) {
            let toplam_maliyet = alim_fiyati * miktar as f64;
            let tarih = chrono_string();

            let id = self.sonraki_alim_id;
            self.alimlar.insert(id, Alim {
                id,
                urun_id,
                miktar,
                alim_fiyati,
                toplam_maliyet,
                tarih,
                kullanici_id,
            });

            urun.miktar += miktar;
            urun.maliyet = alim_fiyati; // Maliyet güncellenir
            self.sonraki_alim_id += 1;

            Ok(id)
        } else {
            Err("Ürün bulunamadı!".to_string())
        }
    }
}

// Güncel tarih-saat string'i oluşturur (chrono bağımlılığı olmadan)
fn chrono_string() -> String {
    let now = std::time::SystemTime::now();
    match now.duration_since(std::time::UNIX_EPOCH) {
        Ok(n) => format!("Timestamp: {}", n.as_secs()),
        Err(_) => "Geçersiz Zaman".to_string(),
    }
}

// Kullanıcı girdisi alma yardımcı fonksiyonu
fn input(prompt: &str) -> String {
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Sayısal kullanıcı girdisi alma yardımcı fonksiyonu
fn input_number<T: std::str::FromStr>(prompt: &str) -> T 
where <T as std::str::FromStr>::Err: std::fmt::Debug {
    loop {
        let input_str = input(prompt);
        match input_str.parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("Geçersiz numara, tekrar deneyin:"),
        }
    }
}

fn ana_menu(magaza: &mut Magaza, kullanici_id: u32) {
    loop {
        println!("\n=== Mağaza Envanter Yönetim Sistemi ===");
        println!("1. Ürün Yönetimi");
        println!("2. Satış İşlemleri");
        println!("3. Alım İşlemleri");
        println!("4. Raporlar");
        if magaza.yonetici_mi(kullanici_id) {
            println!("5. Kullanıcı Yönetimi");
        }
        println!("0. Çıkış");

        let secim = input_number::<u32>("Seçiminiz");
        
        match secim {
            1 => urun_yonetimi(magaza, kullanici_id),
            2 => satis_islemleri(magaza, kullanici_id),
            3 => {
                if magaza.yonetici_mi(kullanici_id) {
                    alim_islemleri(magaza, kullanici_id)
                } else {
                    println!("Bu işlem için yönetici yetkisine sahip olmalısınız!");
                }
            },
            4 => raporlar(magaza, kullanici_id),
            5 => {
                if magaza.yonetici_mi(kullanici_id) {
                    kullanici_yonetimi(magaza)
                } else {
                    println!("Bu işlem için yönetici yetkisine sahip olmalısınız!");
                }
            },
            0 => {
                println!("Çıkış yapılıyor...");
                break;
            },
            _ => println!("Geçersiz seçim!"),
        }
    }
}

fn urun_yonetimi(magaza: &mut Magaza, kullanici_id: u32) {
    loop {
        println!("\n=== Ürün Yönetimi ===");
        println!("1. Ürün Ekle");
        println!("2. Ürün Düzenle");
        println!("3. Ürün Sil");
        println!("4. Ürünleri Listele");
        println!("0. Ana Menüye Dön");

        let secim = input_number::<u32>("Seçiminiz");
        
        match secim {
            1 => {
                if !magaza.yonetici_mi(kullanici_id) {
                    println!("Bu işlem için yönetici yetkisine sahip olmalısınız!");
                    continue;
                }
                let ad = input("Ürün Adı");
                let aciklama = input("Açıklama");
                let fiyat = input_number::<f64>("Satış Fiyatı");
                let miktar = input_number::<u32>("Miktar");
                let maliyet = input_number::<f64>("Maliyet");
                
                let id = magaza.urun_ekle(ad, aciklama, fiyat, miktar, maliyet);
                println!("Ürün başarıyla eklendi! ID: {}", id);
            },
            2 => {
                if !magaza.yonetici_mi(kullanici_id) {
                    println!("Bu işlem için yönetici yetkisine sahip olmalısınız!");
                    continue;
                }
                urunleri_listele(magaza);
                let id = input_number::<u32>("Düzenlenecek Ürün ID");
                
                if let Some(urun) = magaza.urunler.get(&id) {
                    let ad = input(&format!("Yeni Adı [{}]", urun.ad));
                    let aciklama = input(&format!("Yeni Açıklama [{}]", urun.aciklama));
                    let fiyat = input_number::<f64>(&format!("Yeni Fiyat [{}]", urun.fiyat));
                    let miktar = input_number::<u32>(&format!("Yeni Miktar [{}]", urun.miktar));
                    let maliyet = input_number::<f64>(&format!("Yeni Maliyet [{}]", urun.maliyet));
                    
                    if magaza.urun_guncelle(id, ad, aciklama, fiyat, miktar, maliyet) {
                        println!("Ürün başarıyla güncellendi!");
                    } else {
                        println!("Ürün güncellenirken bir hata oluştu!");
                    }
                } else {
                    println!("Ürün bulunamadı!");
                }
            },
            3 => {
                if !magaza.yonetici_mi(kullanici_id) {
                    println!("Bu işlem için yönetici yetkisine sahip olmalısınız!");
                    continue;
                }
                urunleri_listele(magaza);
                let id = input_number::<u32>("Silinecek Ürün ID");
                
                println!("Ürünü silmek istediğinizden emin misiniz? (e/h)");
                let onay = input("");
                if onay.to_lowercase() == "e" {
                    if magaza.urun_sil(id) {
                        println!("Ürün başarıyla silindi!");
                    } else {
                        println!("Ürün bulunamadı!");
                    }
                } else {
                    println!("Silme işlemi iptal edildi.");
                }
            },
            4 => urunleri_listele(magaza),
            0 => break,
            _ => println!("Geçersiz seçim!"),
        }
    }
}

fn urunleri_listele(magaza: &Magaza) {
    println!("\n=== Ürün Listesi ===");
    println!("ID | Adı | Fiyatı | Miktar | Maliyet | Açıklama");
    println!("--------------------------------------------------");
    
    if magaza.urunler.is_empty() {
        println!("Kayıtlı ürün bulunamadı.");
        return;
    }

    for urun in magaza.urunler.values() {
        println!("{} | {} | {:.2} TL | {} | {:.2} TL | {}", 
                 urun.id, urun.ad, urun.fiyat, urun.miktar, urun.maliyet, urun.aciklama);
    }
}

fn satis_islemleri(magaza: &mut Magaza, kullanici_id: u32) {
    println!("\n=== Satış İşlemleri ===");
    urunleri_listele(magaza);
    
    let urun_id = input_number::<u32>("Satılacak Ürün ID");
    
    if let Some(urun) = magaza.urunler.get(&urun_id) {
        println!("Seçilen ürün: {} (Stok: {})", urun.ad, urun.miktar);
        
        let miktar = input_number::<u32>("Satış Miktarı");
        
        match magaza.satis_yap(urun_id, miktar, kullanici_id) {
            Ok(satis_id) => {
                let satis = magaza.satislar.get(&satis_id).unwrap();
                println!("Satış başarıyla gerçekleştirildi!");
                println!("Toplam Tutar: {:.2} TL", satis.toplam_fiyat);
                println!("Kâr: {:.2} TL", satis.kar);
            },
            Err(hata) => println!("Hata: {}", hata),
        }
    } else {
        println!("Ürün bulunamadı!");
    }
}

fn alim_islemleri(magaza: &mut Magaza, kullanici_id: u32) {
    println!("\n=== Alım İşlemleri ===");
    urunleri_listele(magaza);
    
    let urun_id = input_number::<u32>("Alınacak Ürün ID");
    
    if let Some(urun) = magaza.urunler.get(&urun_id) {
        println!("Seçilen ürün: {} (Mevcut Maliyet: {:.2} TL)", urun.ad, urun.maliyet);
        
        let miktar = input_number::<u32>("Alım Miktarı");
        let alim_fiyati = input_number::<f64>("Birim Alım Fiyatı");
        
        match magaza.alim_yap(urun_id, miktar, alim_fiyati, kullanici_id) {
            Ok(alim_id) => {
                let alim = magaza.alimlar.get(&alim_id).unwrap();
                println!("Alım başarıyla gerçekleştirildi!");
                println!("Toplam Maliyet: {:.2} TL", alim.toplam_maliyet);
            },
            Err(hata) => println!("Hata: {}", hata),
        }
    } else {
        println!("Ürün bulunamadı!");
    }
}

fn raporlar(magaza: &Magaza, kullanici_id: u32) {
    loop {
        println!("\n=== Raporlar ===");
        println!("1. Envanter Raporu");
        println!("2. Satış Raporu");
        println!("3. Alım Raporu");
        println!("0. Ana Menüye Dön");

        let secim = input_number::<u32>("Seçiminiz");
        
        match secim {
            1 => envanter_raporu(magaza),
            2 => satis_raporu(magaza),
            3 => {
                if magaza.yonetici_mi(kullanici_id) {
                    alim_raporu(magaza)
                } else {
                    println!("Bu rapora erişmek için yönetici yetkisine sahip olmalısınız!");
                }
            },
            0 => break,
            _ => println!("Geçersiz seçim!"),
        }
    }
}

fn envanter_raporu(magaza: &Magaza) {
    println!("\n=== Envanter Raporu ===");
    println!("ID | Adı | Fiyatı | Miktar | Maliyet | Stok Değeri");
    println!("--------------------------------------------------");
    
    if magaza.urunler.is_empty() {
        println!("Kayıtlı ürün bulunamadı.");
        return;
    }

    let mut toplam_deger = 0.0;
    
    for urun in magaza.urunler.values() {
        let stok_degeri = urun.miktar as f64 * urun.maliyet;
        toplam_deger += stok_degeri;
        
        println!("{} | {} | {:.2} TL | {} | {:.2} TL | {:.2} TL", 
                 urun.id, urun.ad, urun.fiyat, urun.miktar, urun.maliyet, stok_degeri);
    }
    
    println!("\nToplam Envanter Değeri: {:.2} TL", toplam_deger);
}

fn satis_raporu(magaza: &Magaza) {
    println!("\n=== Satış Raporu ===");
    println!("ID | Ürün | Miktar | Birim Fiyat | Toplam | Kâr | Tarih");
    println!("--------------------------------------------------");
    
    if magaza.satislar.is_empty() {
        println!("Kayıtlı satış bulunamadı.");
        return;
    }

    let mut toplam_satis = 0.0;
    let mut toplam_kar = 0.0;
    
    for satis in magaza.satislar.values() {
        let urun_adi = magaza.urunler.get(&satis.urun_id)
            .map_or("Bilinmeyen Ürün", |u| &u.ad);
            
        println!("{} | {} | {} | {:.2} TL | {:.2} TL | {:.2} TL | {}", 
                 satis.id, urun_adi, satis.miktar, satis.satis_fiyati, 
                 satis.toplam_fiyat, satis.kar, satis.tarih);
                 
        toplam_satis += satis.toplam_fiyat;
        toplam_kar += satis.kar;
    }
    
    println!("\nToplam Satış: {:.2} TL", toplam_satis);
    println!("Toplam Kâr: {:.2} TL", toplam_kar);
}

fn alim_raporu(magaza: &Magaza) {
    println!("\n=== Alım Raporu ===");
    println!("ID | Ürün | Miktar | Birim Fiyat | Toplam Maliyet | Tarih");
    println!("--------------------------------------------------");
    
    if magaza.alimlar.is_empty() {
        println!("Kayıtlı alım bulunamadı.");
        return;
    }

    let mut toplam_maliyet = 0.0;
    
    for alim in magaza.alimlar.values() {
        let urun_adi = magaza.urunler.get(&alim.urun_id)
            .map_or("Bilinmeyen Ürün", |u| &u.ad);
            
        println!("{} | {} | {} | {:.2} TL | {:.2} TL | {}", 
                 alim.id, urun_adi, alim.miktar, alim.alim_fiyati, 
                 alim.toplam_maliyet, alim.tarih);
                 
        toplam_maliyet += alim.toplam_maliyet;
    }
    
    println!("\nToplam Alım Maliyeti: {:.2} TL", toplam_maliyet);
}

fn kullanici_yonetimi(magaza: &mut Magaza) {
    loop {
        println!("\n=== Kullanıcı Yönetimi ===");
        println!("1. Kullanıcı Ekle");
        println!("2. Kullanıcıları Listele");
        println!("0. Ana Menüye Dön");

        let secim = input_number::<u32>("Seçiminiz");
        
        match secim {
            1 => {
                let kullanici_adi = input("Kullanıcı Adı");
                let sifre = input("Şifre");
                
                println!("Kullanıcı yönetici mi? (e/h)");
                let rol_input = input("");
                let rol = if rol_input.to_lowercase() == "e" {
                    KullaniciRolu::Yonetici
                } else {
                    KullaniciRolu::Calisan
                };
                
                let id = magaza.kullanici_olustur(kullanici_adi, sifre, rol);
                if id > 0 {
                    println!("Kullanıcı başarıyla oluşturuldu! ID: {}", id);
                }
            },
            2 => kullanicilari_listele(magaza),
            0 => break,
            _ => println!("Geçersiz seçim!"),
        }
    }
}

fn kullanicilari_listele(magaza: &Magaza) {
    println!("\n=== Kullanıcı Listesi ===");
    println!("ID | Kullanıcı Adı | Rol");
    println!("-------------------------");
    
    if magaza.kullanicilar.is_empty() {
        println!("Kayıtlı kullanıcı bulunamadı.");
        return;
    }

    for (id, kullanici) in &magaza.kullanicilar {
        let rol = match kullanici.rol {
            KullaniciRolu::Yonetici => "Yönetici",
            KullaniciRolu::Calisan => "Çalışan",
        };
        
        println!("{} | {} | {}", id, kullanici.kullanici_adi, rol);
    }
}

fn main() {
    println!("=== Mağaza Envanter Yönetim Sistemi ===");
    println!("Version: 1.0.0");
    
    let mut magaza = Magaza::new();
    println!("Varsayılan yönetici hesabı oluşturuldu.");
    println!("Kullanıcı adı: admin");
    println!("Şifre: admin");
    
    loop {
        println!("\nLütfen giriş yapın:");
        let kullanici_adi = input("Kullanıcı Adı");
        let sifre = input("Şifre");
        
        match magaza.kullanici_girisi(&kullanici_adi, &sifre) {
            Some(kullanici_id) => {
                println!("Giriş başarılı!");
                ana_menu(&mut magaza, kullanici_id);
                println!("Hoşçakalın!");
                break;
            },
            None => println!("Geçersiz kullanıcı adı veya şifre! Tekrar deneyin."),
        }
    }
}
