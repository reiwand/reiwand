use crate::backend::qobject::Backend;
use crate::{choices, texts};
use cxx_qt_lib::QString;

impl Backend {
    pub fn licenses(&self) -> QString {
        QString::from(texts::LICENSES)
    }

    pub fn markup(&self, text: &QString) -> QString {
        QString::from(&reiwand_core::markup::html(&text.to_string()))
    }

    pub fn plain(&self, text: &QString) -> QString {
        QString::from(&reiwand_core::markup::plain(&text.to_string()))
    }

    pub fn toggle(&self, list: &QString, item: &QString, order: &QString) -> QString {
        QString::from(&choices::toggled(&list.to_string(), &item.to_string(), &order.to_string()))
    }

    pub fn flip(&self, bits: &QString, index: i32) -> QString {
        QString::from(&choices::flipped(&bits.to_string(), usize::try_from(index).unwrap_or_default()))
    }
}
