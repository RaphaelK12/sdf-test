// ============ Absolute Layout =========================================================

use super::widget::{UILayout, UIPoint, UISize, UIWidget};

#[derive(Copy, Clone)]
pub struct UIAbsoluteLayout {
    pub size: UISize,
    pub pos: UIPoint,
}

impl UIWidget for UIAbsoluteLayout {
    type Event = ();
    fn layout(&self, layout: UILayout, children: &mut [UILayout]) {
        for child in children {
            child.left = layout.left + self.pos.left;
            child.top = layout.top + self.pos.top;
            child.width = self.size.width;
            child.height = self.size.height;
        }
    }
}

// ============ Relative Layout =========================================================

#[derive(Copy, Clone)]
pub struct UIRelativeLayout {
    pub size: UISize,
    pub pos: UIPoint,
}

impl UIWidget for UIRelativeLayout {
    type Event = ();
    fn layout(&self, layout: UILayout, children: &mut [UILayout]) {
        for child in children {
            child.left = layout.left + layout.width * self.pos.left;
            child.top = layout.top + layout.height * self.pos.top;
            child.width = layout.width * self.size.width;
            child.height = layout.height * self.size.height;
        }
    }
}

// ============ Scale Layout =========================================================

#[derive(Copy, Clone)]
pub struct UIScaleLayout {
    pub scale: UISize,
    pub anchor: UIPoint,
}

impl UIWidget for UIScaleLayout {
    type Event = ();
    fn layout(&self, layout: UILayout, children: &mut [UILayout]) {
        let origin_left = self.anchor.left * layout.width + layout.left;
        let origin_top = self.anchor.top * layout.height + layout.top;

        for child in children {
            child.left = (layout.left - origin_left) * self.scale.width + origin_left;
            child.top = (layout.top - origin_top) * self.scale.height + origin_top;
            child.width = layout.width * self.scale.width;
            child.height = layout.height * self.scale.height;
        }
    }
}

// ============ Main Layout =========================================================

#[derive(Copy, Clone)]
pub struct UIMainLayout {
    pub min_width: f32,
    pub max_width: f32,
    pub ratio: f32,
    pub padding: f32,
}

impl UIWidget for UIMainLayout {
    type Event = ();
    fn layout(&self, layout: UILayout, children: &mut [UILayout]) {
        if children.len() != 3 {
            panic!("Expected 3 children in main layout!");
        }

        let drawer_width = (layout.width * self.ratio)
            .max(self.min_width)
            .min(self.max_width);

        children[0] = UILayout {
            left: self.padding,
            top: self.padding,
            height: layout.height - 2.0 * self.padding,
            width: drawer_width - 2.0 * self.padding,
        };

        children[1] = UILayout {
            left: drawer_width,
            top: self.padding,
            height: layout.height - 2.0 * self.padding,
            width: layout.width - 2.0 * drawer_width,
        };

        children[2] = UILayout {
            left: layout.width - drawer_width + self.padding,
            top: self.padding,
            height: layout.height - 2.0 * self.padding,
            width: drawer_width - 2.0 * self.padding,
        };
    }
}

// ============ VBox Layout =========================================================

#[derive(Copy, Clone)]
pub struct UIVBoxLayout {
    pub hpadding: f32,
    pub vpadding: f32,
    pub min_height: f32,
    pub max_height: f32,
}

impl UIWidget for UIVBoxLayout {
    type Event = ();
    fn layout(&self, layout: UILayout, children: &mut [UILayout]) {
        let height = ((layout.height - (children.len() + 1) as f32 * self.vpadding)
            / children.len() as f32)
            .min(self.max_height)
            .max(self.min_height);

        for (index, child) in children.iter_mut().enumerate() {
            child.left = layout.left + self.hpadding;
            child.width = layout.width - self.hpadding * 2.0;
            child.height = height;
            child.top =
                layout.top + layout.height - (index as f32 + 1.0) * (self.vpadding + height);
        }
    }
}

// ============ HBox Layout =========================================================

#[derive(Copy, Clone)]
pub struct UIHBoxLayout {
    pub hpadding: f32,
    pub vpadding: f32,
    pub min_width: f32,
    pub max_width: f32,
}

impl UIWidget for UIHBoxLayout {
    type Event = ();
    fn layout(&self, layout: UILayout, children: &mut [UILayout]) {
        let width = ((layout.width - (children.len() + 1) as f32 * self.hpadding)
            / children.len() as f32)
            .min(self.max_width)
            .max(self.min_width);

        for (index, child) in children.iter_mut().enumerate() {
            child.left = layout.left + self.hpadding + index as f32 * width;
            child.width = width;
            child.height = layout.height - 2.0 * self.vpadding;
            child.top = layout.top + self.vpadding;
        }
    }
}

// ============ Slider Layout =========================================================

#[derive(Copy, Clone)]
pub struct UISliderLayout {
    pub label_offset: f32,
}

impl UIWidget for UISliderLayout {
    type Event = ();
    fn layout(&self, layout: UILayout, children: &mut [UILayout]) {
        if children.len() != 2 {
            panic!("Expected 2 children in main layout!");
        }

        children[0] = layout;
        children[1] = UILayout {
            top: layout.top - self.label_offset,
            ..layout
        };
    }
}
