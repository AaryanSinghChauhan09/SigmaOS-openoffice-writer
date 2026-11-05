#[derive(Debug, Clone, PartialEq)]
pub struct Paragraph {
    pub text: String,
    pub style: String,
}

pub struct WriterDocument {
    pub paragraphs: Vec<Paragraph>,
}

impl Default for WriterDocument {
    fn default() -> Self {
        Self::new()
    }
}

impl WriterDocument {
    pub fn new() -> Self {
        Self { paragraphs: Vec::new() }
    }

    pub fn add_paragraph(&mut self, text: impl Into<String>, style: impl Into<String>) {
        self.paragraphs.push(Paragraph { text: text.into(), style: style.into() });
    }

    pub fn word_count(&self) -> usize {
        self.paragraphs.iter().map(|p| p.text.split_whitespace().count()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_writer_document() {
        // Arrange
        let mut doc = WriterDocument::new();
        // Act
        doc.add_paragraph("Hello world from SigmaOS Writer", "Heading1");
        // Assert
        assert_eq!(doc.word_count(), 5);
    }
}
