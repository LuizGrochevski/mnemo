// Implementação do algoritmo SM-2 (o mesmo usado pelo Anki original)
// quality vai de 0 a 5: quanto o usuário lembrou bem do card
pub fn sm2(quality: u8, repetition: i32, ease_factor: f64, interval: i32) -> (i32, i32, f64) {
    let mut ef = ease_factor;
    let mut rep = repetition;
    let mut int = interval;

    if quality < 3 {
        // Errou ou lembrou mal: reseta a repetição, revisa amanhã
        rep = 0;
        int = 1;
    } else {
        int = match rep {
            0 => 1,
            1 => 6,
            _ => (int as f64 * ef).round() as i32,
        };
        rep += 1;
    }

    // Ajusta o "fator de facilidade" baseado na qualidade da resposta
    ef = ef + (0.1 - (5.0 - quality as f64) * (0.08 + (5.0 - quality as f64) * 0.02));
    if ef < 1.3 {
        ef = 1.3;
    }

    (int, rep, ef)
}
