use gpui::{
    px, IntoElement, ParentElement as _, Render, Styled as _, View, ViewContext,
    VisualContext as _, WindowContext,
};
use ui::{
    date_picker::{DatePicker, DatePickerEvent},
    v_flex, Sizable as _,
};

pub struct CalendarStory {
    date_picker: View<DatePicker>,
    date_picker_small: View<DatePicker>,
    date_picker_large: View<DatePicker>,
    date_picker_value: Option<String>,
}

impl CalendarStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(Self::new)
    }

    fn new(cx: &mut ViewContext<Self>) -> Self {
        let date_picker = cx.new_view(|cx| {
            DatePicker::new("date_picker_medium", cx)
                .default_now()
                .cleanable(true)
                .width(px(220.))
        });
        let date_picker_large = cx.new_view(|cx| {
            DatePicker::new("date_picker_large", cx)
                .large()
                .date_format("%Y-%m-%d")
                .width(px(300.))
        });
        let date_picker_small = cx.new_view(|cx| {
            DatePicker::new("date_picker_small", cx)
                .default_now()
                .small()
                .width(px(180.))
        });

        cx.subscribe(&date_picker, |this, _, ev, _| match ev {
            DatePickerEvent::Change(date) => {
                this.date_picker_value = date.map(|d| d.to_string());
            }
        })
        .detach();

        Self {
            date_picker,
            date_picker_large,
            date_picker_small,
            date_picker_value: None,
        }
    }
}

impl Render for CalendarStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(self.date_picker.clone())
            .child(self.date_picker_small.clone())
            .child(self.date_picker_large.clone())
            .child(format!("Date picker value: {:?}", self.date_picker_value).into_element())
    }
}