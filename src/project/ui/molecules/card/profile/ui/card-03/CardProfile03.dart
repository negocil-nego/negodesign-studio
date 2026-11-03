import 'package:flutter/material.dart';
import '../../data/types.dart';
import '../../../shared/card-actions-button/ui/CardActionsButton.dart';
import '../../../shared/card-description/ui/CardDescription.dart';
import '../../../shared/card-image/ui/CardImage.dart';
import '../../../shared/card-logo/ui/CardLogo.dart';
import '../../../shared/card-tags/ui/CardTags.dart';
import '../../../shared/card-thumbnail-video/ui/CardThumbnailVideo.dart';
import '../../../shared/card-title/ui/CardTitle.dart';
import '../../../shared/card-video/ui/CardVideo.dart';
import '../../../shared/card-video/data/types.dart';

class CardProfile03 extends StatelessWidget {
  final CardProfileV03Props props;

  const CardProfile03(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 352,
      clipBehavior: Clip.antiAlias,
      decoration: BoxDecoration(
        color: Theme.of(context).cardColor,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: const Color(0xFFE4E4E7)),
        boxShadow: const [
          BoxShadow(color: Color(0x0A000000), blurRadius: 4, offset: Offset(0, 1)),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        mainAxisSize: MainAxisSize.min,
        children: [
          // ── Cover ──────────────────────────────────────────────────
          SizedBox(
            height: 128,
            child: Stack(
              clipBehavior: Clip.none,
              children: [
                Positioned.fill(child: _buildMedia()),

                // Edit Profile button — top-right of cover
                if (props.editActionProps != null)
                  Positioned(
                    top: 12,
                    right: 12,
                    child: CardActionsButton(
                      props.editActionProps!,
                      variant: CardActionsButtonVariant.iconOutline,
                    ),
                  ),

                // Avatar anchored to bottom of cover
                if (props.logoProps != null)
                  Positioned(
                    bottom: -40,
                    left: 20,
                    child: Container(
                      width: 80,
                      height: 80,
                      clipBehavior: Clip.antiAlias,
                      decoration: BoxDecoration(
                        color: Colors.white,
                        shape: BoxShape.circle,
                        border: Border.all(color: Colors.white, width: 4),
                        boxShadow: const [
                          BoxShadow(color: Color(0x26000000), blurRadius: 6),
                        ],
                      ),
                      child: CardLogo(props.logoProps!),
                    ),
                  ),
              ],
            ),
          ),

          // ── Body ───────────────────────────────────────────────────
          Padding(
            padding: const EdgeInsets.fromLTRB(20, 48, 20, 20),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                // Name + handle row alongside Follow / Message buttons
                Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          if (props.titleProps != null) CardTitle(props.titleProps!),
                          if (props.handleProps != null)
                            Text(
                              props.handleProps!,
                              style: const TextStyle(
                                fontSize: 14,
                                color: Color(0xFF71717A),
                              ),
                            ),
                        ],
                      ),
                    ),
                    if (props.actionsProps != null)
                      CardActionsButton(
                        props.actionsProps!,
                        variant: CardActionsButtonVariant.mixed,
                      ),
                  ],
                ),

                const SizedBox(height: 16),

                // About / Description
                if (props.descriptionProps != null || props.tagsProps != null) ...[
                  const Text(
                    'About',
                    style: TextStyle(
                      fontSize: 14,
                      fontWeight: FontWeight.w600,
                      color: Color(0xFF18181B),
                    ),
                  ),
                  const SizedBox(height: 4),
                  if (props.descriptionProps != null)
                    CardDescription(props.descriptionProps!),
                  const SizedBox(height: 16),
                ],

                // Stats
                if (props.tagsProps != null)
                  CardTags(
                    CardTagsProps(
                      tags: props.tagsProps!.tags,
                      variant: 'stats',
                    ),
                  ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildMedia() {
    if (props.thumbnailVideoProps != null) {
      return CardThumbnailVideo(props.thumbnailVideoProps!);
    }
    if (props.videoProps != null) {
      final video = props.videoProps!;
      return CardVideo(
        CardVideoProps(
          src: video.src,
          poster: video.poster,
          preload: video.preload,
          controls: false,
        ),
      );
    }
    if (props.imageProps != null) return CardImage(props.imageProps!);

    // Gradient cover fallback (matches design)
    return Container(
      decoration: const BoxDecoration(
        gradient: LinearGradient(
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
          colors: [Color(0xFFFBCFE8), Color(0xFFFDE68A), Color(0xFFFED7AA)],
        ),
      ),
    );
  }
}

