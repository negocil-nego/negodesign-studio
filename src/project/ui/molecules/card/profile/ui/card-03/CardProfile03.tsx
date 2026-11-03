// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
import CardActionsButton from "../../../shared/card-actions-button/ui/CardActionsButton";
import CardDescription from "../../../shared/card-description/ui/CardDescription";
import CardImage from "../../../shared/card-image/ui/CardImage";
import CardLogo from "../../../shared/card-logo/ui/CardLogo";
import CardTags from "../../../shared/card-tags/ui/CardTags";
import CardThumbnailVideo from "../../../shared/card-thumbnail-video/ui/CardThumbnailVideo";
import CardTitle from "../../../shared/card-title/ui/CardTitle";
import CardVideo from "../../../shared/card-video/ui/CardVideo";
import type { CardProfileBaseProps } from "../../data/types";

export default function CardProfile03({
    logoProps,
    titleProps,
    handleProps,
    videoProps,
    imageProps,
    thumbnailVideoProps,
    descriptionProps,
    tagsProps,
    actionsProps,
    editActionProps,
}: CardProfileBaseProps) {
    const style = StyleComponent.card.profile.v03;
    return (
        <aside className={style.root}>
            {/* Cover */}
            <div className={style.cover}>
                {thumbnailVideoProps ? (
                    <CardThumbnailVideo {...thumbnailVideoProps} />
                ) : videoProps ? (
                    <CardVideo {...videoProps} controls={false} />
                ) : imageProps ? (
                    <CardImage {...imageProps} className={style.coverMedia} />
                ) : (
                    <div className={style.coverFallback} />
                )}

                {/* Edit Profile button — top-right of cover */}
                {editActionProps && (
                    <div className={style.editAction}>
                        <CardActionsButton actions={editActionProps.actions} variant="icon-outline" />
                    </div>
                )}

                {/* Avatar anchored to bottom of cover */}
                {logoProps && (
                    <div className={style.logoWrapper}>
                        <CardLogo {...logoProps} className={style.logoImage} />
                    </div>
                )}
            </div>

            {/* Body */}
            <div className={style.body}>
                {/* Name + handle row alongside Follow / Message buttons */}
                <div className={style.headRow}>
                    <div className={style.nameGroup}>
                        {titleProps && <CardTitle {...titleProps} className={style.name} />}
                        {handleProps && <span className={style.handle}>{handleProps}</span>}
                    </div>

                    {actionsProps && (
                        <div className={style.actionsGroup}>
                            <CardActionsButton actions={actionsProps.actions} variant="mixed" />
                        </div>
                    )}
                </div>

                {/* About / Description */}
                {(descriptionProps || tagsProps) && (
                    <div className={style.aboutSection}>
                        <span className={style.aboutLabel}>About</span>
                        {descriptionProps && (
                            <CardDescription {...descriptionProps} className={style.description} />
                        )}
                    </div>
                )}

                {/* Stats */}
                {tagsProps && (
                    <CardTags
                        {...tagsProps}
                        variant="stats"
                        className={style.stats}
                        itemClassName={style.statItem}
                        titleClassName={style.statValue}
                        descriptionClassName={style.statLabel}
                    />
                )}
            </div>
        </aside>
    );
}

