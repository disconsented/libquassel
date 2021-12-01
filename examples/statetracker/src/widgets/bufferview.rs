use std::sync::Arc;

use druid::widget::{Label};
use druid::{lens, Lens, Point, WidgetPod};
use druid::{widget::Flex, Widget};

use libquassel::message::objects::BufferViewManager;

pub struct BufferViewWidget {
    inner: WidgetPod<Arc<BufferViewManager>, Box<dyn Widget<Arc<BufferViewManager>>>>,
}

impl BufferViewWidget {
    pub fn new() -> Self {
        let widget = WidgetPod::new(Flex::column()).boxed();

        BufferViewWidget { inner: widget }
    }
}

impl Widget<Arc<BufferViewManager>> for BufferViewWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Arc<BufferViewManager>,
        env: &druid::Env,
    ) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &Arc<BufferViewManager>,
        env: &druid::Env,
    ) {
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        _old_data: &Arc<BufferViewManager>,
        data: &Arc<BufferViewManager>,
        _env: &druid::Env,
    ) {
        let buffer_view_configs = lens!(BufferViewManager, buffer_view_configs);

        let mut names: Flex<Arc<BufferViewManager>> = Flex::row();
        let mut buffers: Flex<Arc<BufferViewManager>> = Flex::row();
        let mut removed_buffers: Flex<Arc<BufferViewManager>> = Flex::row();
        let mut temporarily_removed_buffers: Flex<Arc<BufferViewManager>> = Flex::row();
        // let mut expansions: Flex<Arc<BufferViewManager>> = Flex::column();

        // TODO optimise this whole thing
        buffer_view_configs.with(data, |configs| {
            for (_id, config) in configs {
                names.add_child(Label::new(config.buffer_view_name.clone()));
                buffers.add_child(Label::new(format!("{:?}", config.buffers)));
                removed_buffers.add_child(Label::new(format!("{:?}", config.removed_buffers)));
                temporarily_removed_buffers.add_child(Label::new(format!(
                    "{:?}",
                    config.temporarily_removed_buffers
                )));
                // expansions.add_child(Align::left(Label::new(alias.expansion.clone())));
            }
        });

        let widget: Flex<Arc<BufferViewManager>> = Flex::column()
            .with_flex_child(names, 1.0)
            .with_flex_child(buffers, 1.0)
            .with_flex_child(removed_buffers, 1.0)
            .with_flex_child(temporarily_removed_buffers, 1.0);
        //     .with_flex_child(expansions, 1.0);

        self.inner = WidgetPod::new(widget).boxed();

        ctx.children_changed();
        ctx.request_layout();
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &Arc<BufferViewManager>,
        env: &druid::Env,
    ) -> druid::Size {
        let size = self.inner.layout(ctx, bc, data, env);
        self.inner.set_origin(ctx, data, env, Point::ZERO);
        return size;
    }

    fn paint(
        &mut self,
        ctx: &mut druid::PaintCtx,
        data: &Arc<BufferViewManager>,
        env: &druid::Env,
    ) {
        self.inner.paint(ctx, data, env)
    }
}
